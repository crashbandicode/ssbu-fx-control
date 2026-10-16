//! ACMD has its own request path; lua_bind::EffectModule hooks do not cover it.
//! Mirrors the supplied FX mod's stack bridge, with bounded primitive storage,
//! no per-frame allocation and an unchanged original call after substitution.
//! The 0.1.x `EFFECT_OFF_KIND` / `EFFECT_DETACH_KIND` cleanup hooks (ids
//! 0x11a / 0x11b) left with the EFF siblings in 0.2.0.
//!
//! Ported from Online Deluxe `src/runtime/acmd.rs`: each effect name takes the
//! mode of the fighter that owns it (`crate::policy`).
use super::{record, Code, TABLE};
use crate::{acmd::Value, effects_reduced::FxIntent, policy};
use std::sync::atomic::{AtomicU64, Ordering};

#[repr(C)]
struct Agent {
    // skyline-smash bed642e L2CAgent (cpp.rs), 0x48 bytes, alignment 8.
    storage: [u64; 9],
}

extern "C" {
    #[link_name = "\u{1}_ZN3lib8L2CAgentC2EP9lua_State"]
    fn agent_construct(agent: *mut Agent, lua_state: u64) -> *mut Agent;
    #[link_name = "\u{1}_ZN3lib8L2CAgent13pop_lua_stackEi"]
    fn stack_value(agent: *mut Agent, index: i32) -> Value;
    #[link_name = "\u{1}_ZN3lib8L2CAgent14push_lua_stackERKNS_8L2CValueE"]
    fn stack_push(agent: *mut Agent, value: *const Value) -> u64;
    #[link_name = "\u{1}_ZN3lib8L2CAgent15clear_lua_stackEv"]
    fn stack_clear(agent: *mut Agent) -> u64;
}

static REJECTED: AtomicU64 = AtomicU64::new(0);

fn reject(path: u64, reason: u64) {
    let count = REJECTED.fetch_add(1, Ordering::Relaxed);
    if count < 8 {
        record(Code::AcmdStackRejected, path, reason);
    }
}

unsafe fn filter(lua_state: u64, path: u64, calls: &AtomicU64, hash_count: usize) {
    if lua_state == 0 || !TABLE.any_active() {
        return;
    }
    let scope = super::scope_allows();
    if TABLE.idle(scope) {
        return;
    }
    let count = calls.fetch_add(1, Ordering::Relaxed) + 1;
    let mut agent = Agent { storage: [0; 9] };
    agent_construct(&mut agent, lua_state);
    let mut args = [Value::VOID; 32];
    for (index, arg) in args.iter_mut().take(hash_count).enumerate() {
        *arg = stack_value(&mut agent, (index + 1) as i32);
        if !matches!(arg.kind, 2 | 7) {
            reject(path, arg.kind as u64);
            return;
        }
    }
    if count == 1 || count.is_multiple_of(256) {
        record(Code::AcmdEffectSeen, path, args[0].raw);
    }
    let context = if args[..hash_count]
        .iter()
        .any(|v| policy::needs_context(v.raw))
    {
        super::effect_context::from_lua(lua_state)
    } else {
        None
    };
    let mut intents = [FxIntent::Passthrough; 2];
    let mut forwarded = [0u64; 2];
    let mut changed = false;
    for index in 0..hash_count {
        let decision = policy::decide(&TABLE, scope, args[index].raw, context);
        intents[index] = decision.intent;
        forwarded[index] = decision.forwarded;
        if decision.forwarded != args[index].raw {
            changed = true;
        }
    }
    // Reads don't clear Lua. A rejected tail leaves the original request untouched.
    let mut len = hash_count;
    if changed && scope {
        loop {
            let value = stack_value(&mut agent, (len + 1) as i32);
            if value.kind == 0 {
                break;
            }
            if len == args.len() || !value.is_primitive() {
                reject(path, value.kind as u64);
                return;
            }
            args[len] = value;
            len += 1;
        }
    }
    let originals = [args[0].raw, args[1].raw];
    for index in 0..hash_count {
        let intent = intents[index];
        let next = forwarded[index];
        let code = match intent {
            FxIntent::Passthrough => {
                if changed && scope {
                    args[index].raw = next;
                }
                continue;
            }
            FxIntent::Suppress => Code::AcmdEffectSuppressed,
            FxIntent::ReducedCull => Code::FxReducedCull,
            FxIntent::ReducedKeep => Code::FxReducedKeep,
            FxIntent::ReducedRestore => Code::FxReducedRestore,
            FxIntent::ReducedPreserve => Code::FxReducedPreserve,
        };
        let total = crate::effects_reduced::observe(intent, false);
        let sample = match intent {
            FxIntent::Suppress => total <= 32 || total.is_multiple_of(128),
            _ => total == 1 || total.is_multiple_of(128),
        };
        if sample {
            record(code, originals[index], (path << 32) | total);
        }
        if changed && scope {
            args[index].raw = next;
        }
    }
    if changed && scope {
        stack_clear(&mut agent);
        for value in &args[..len] {
            stack_push(&mut agent, value);
        }
    }
}

macro_rules! effect_hook {
    ($target:ident, $hook:ident, $symbol:literal, $id:expr) => {
        effect_hook!($target, $hook, $symbol, $id, 1);
    };
    ($target:ident, $hook:ident, $symbol:literal, $id:expr, $arity:expr) => {
        extern "C" {
            #[link_name = $symbol]
            fn $target(lua_state: u64);
        }
        #[skyline::hook(replace = $target)]
        unsafe fn $hook(lua_state: u64) {
            static CALLS: AtomicU64 = AtomicU64::new(0);
            filter(lua_state, $id, &CALLS, $arity);
            call_original!(lua_state);
        }
    };
}

// Verified vanilla Palutena Explode and Edge Burst/Flare scripts use EFFECT
// and EFFECT_FOLLOW. Alpha/no-stop/position variants cover the existing policy.
effect_hook!(
    effect,
    ssbu_fx_control_filter_effect,
    "\u{1}_ZN3app10sv_animcmd6EFFECTEP9lua_State",
    0x110
);
effect_hook!(
    follow,
    ssbu_fx_control_filter_follow,
    "\u{1}_ZN3app10sv_animcmd13EFFECT_FOLLOWEP9lua_State",
    0x111
);
effect_hook!(
    alpha,
    ssbu_fx_control_filter_alpha,
    "\u{1}_ZN3app10sv_animcmd19EFFECT_FOLLOW_ALPHAEP9lua_State",
    0x112
);
effect_hook!(
    no_stop,
    ssbu_fx_control_filter_no_stop,
    "\u{1}_ZN3app10sv_animcmd21EFFECT_FOLLOW_NO_STOPEP9lua_State",
    0x113
);
effect_hook!(
    pos,
    ssbu_fx_control_filter_pos,
    "\u{1}_ZN3app10sv_animcmd14EFFECT_FLW_POSEP9lua_State",
    0x114
);
effect_hook!(
    pos_no_stop,
    ssbu_fx_control_filter_pos_no_stop,
    "\u{1}_ZN3app10sv_animcmd22EFFECT_FLW_POS_NO_STOPEP9lua_State",
    0x115
);

effect_hook!(
    landing,
    ssbu_fx_control_filter_landing,
    "\u{1}_ZN3app10sv_animcmd14LANDING_EFFECTEP9lua_State",
    0x116
);
effect_hook!(
    flip,
    ssbu_fx_control_filter_flip,
    "\u{1}_ZN3app10sv_animcmd18EFFECT_FOLLOW_FLIPEP9lua_State",
    0x117,
    2
);
effect_hook!(
    flip_alpha,
    ssbu_fx_control_filter_flip_alpha,
    "\u{1}_ZN3app10sv_animcmd24EFFECT_FOLLOW_FLIP_ALPHAEP9lua_State",
    0x118,
    2
);
effect_hook!(
    flip_no_stop,
    ssbu_fx_control_filter_flip_no_stop,
    "\u{1}_ZN3app10sv_animcmd26EFFECT_FOLLOW_NO_STOP_FLIPEP9lua_State",
    0x119,
    2
);

pub(super) fn install() {
    macro_rules! install {
        ($target:ident, $hook:ident, $id:expr) => {
            let address = $target as *const () as u64;
            if !unsafe { super::memory::accessible(address as usize, 4, false) } {
                record(Code::HookRejected, $id, address);
            } else {
                record(Code::HookInstalling, $id, address);
                let word = unsafe { *(address as *const u32) } as u64;
                record(Code::HookEntryWord, $id, word);
                super::logging::install_checkpoint(Code::HookEntryWord, $id, word);
                skyline::install_hook!($hook);
                record(Code::HookInstalled, $id, address);
            }
        };
    }
    install!(effect, ssbu_fx_control_filter_effect, 0x110);
    install!(follow, ssbu_fx_control_filter_follow, 0x111);
    install!(alpha, ssbu_fx_control_filter_alpha, 0x112);
    install!(no_stop, ssbu_fx_control_filter_no_stop, 0x113);
    install!(pos, ssbu_fx_control_filter_pos, 0x114);
    install!(pos_no_stop, ssbu_fx_control_filter_pos_no_stop, 0x115);
    install!(landing, ssbu_fx_control_filter_landing, 0x116);
    install!(flip, ssbu_fx_control_filter_flip, 0x117);
    install!(flip_alpha, ssbu_fx_control_filter_flip_alpha, 0x118);
    install!(flip_no_stop, ssbu_fx_control_filter_flip_no_stop, 0x119);
}
