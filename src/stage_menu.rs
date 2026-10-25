//! Second own-page descriptor from the same FX Control NRO. Its All row
//! controls stage backgrounds only, in both offline and online play.
//! The plugin's existing background saver owns all stage-key persistence.
use crate::{config::FxMode, policy::ModeTable, stages};
use deluxe_plugin_sdk::{OptionSpec, PluginSpec, RequestStatus, RowLayout, Tone};

pub const FIRST_STAGE: usize = 2;
pub const OPTION_COUNT: usize = FIRST_STAGE + stages::COUNT + 1;
static CHOICES: [&str; 2] = ["On", "Off"];
static TONES: [Tone; 2] = [Tone::Ok, Tone::Neutral];

const fn build_options() -> [OptionSpec; OPTION_COUNT] {
    let value = OptionSpec {
        id: "all",
        label: "All backgrounds",
        choices: &CHOICES,
        config_key: None,
        values: None,
        no_persist: true,
        tones: Some(&TONES),
        read_only: false,
        layout: RowLayout::Value,
    };
    let mut options = [value; OPTION_COUNT];
    options[0] = OptionSpec::header("backgrounds", "Background visuals");
    let mut index = 0;
    while index < stages::COUNT {
        options[FIRST_STAGE + index] = OptionSpec {
            id: stages::ALL[index].key,
            label: stages::ALL[index].label,
            ..value
        };
        index += 1;
    }
    options[OPTION_COUNT - 1] = OptionSpec::header("reload", "Reload stage to apply");
    options
}

pub static OPTIONS: [OptionSpec; OPTION_COUNT] = build_options();
pub static SPEC: PluginSpec = PluginSpec {
    id: "fx_control_stages",
    label: "Stages",
    options: &OPTIONS,
    config_path: None,
    own_page: true,
};

fn required(option: usize) -> Option<u16> {
    match option {
        1 => Some(stages::SUPPORTED_MASK),
        index if (FIRST_STAGE..FIRST_STAGE + stages::COUNT).contains(&index) => {
            Some(1 << (index - FIRST_STAGE))
        }
        _ => None,
    }
}

pub fn current(table: &ModeTable, option: usize, available: u16) -> Option<usize> {
    let required = required(option)?;
    if available & required != required {
        return None;
    }
    match option {
        1 => stages::common_mode(&table.stage_modes()).map(|mode| mode as usize),
        index if (FIRST_STAGE..FIRST_STAGE + stages::COUNT).contains(&index) => table
            .stage_mode(index - FIRST_STAGE)
            .map(|mode| mode as usize),
        _ => None,
    }
}

pub fn request(table: &ModeTable, option: usize, choice: usize, available: u16) -> RequestStatus {
    if option == 0 || option >= FIRST_STAGE + stages::COUNT || choice >= CHOICES.len() {
        return RequestStatus::Rejected;
    }
    let Some(required) = required(option) else {
        return RequestStatus::Rejected;
    };
    if available & required != required {
        return RequestStatus::Unavailable;
    }
    let mode = if choice == 0 { FxMode::On } else { FxMode::Off };
    if option == 1 {
        table.set_all_stages(mode);
    } else {
        table.set_stage(option - FIRST_STAGE, mode);
    }
    // The policy is active now, but existing particles are not deleted and
    // model visibility is not replayed: the page asks for a stage reload.
    RequestStatus::Applied
}

#[cfg(test)]
mod tests {
    use super::*;
    use deluxe_plugin_sdk::{register, PluginHandler, RegisterError};

    struct Handler;
    impl PluginHandler for Handler {
        fn current(&self, _: usize) -> Option<usize> {
            Some(0)
        }
        fn request(&self, _: usize, _: usize) -> RequestStatus {
            RequestStatus::Rejected
        }
    }

    #[test]
    fn descriptor_is_valid_and_keeps_the_single_nro_pages_distinct() {
        assert_eq!(register(&SPEC, Handler), Err(RegisterError::HostMissing));
        assert_ne!(SPEC.id, crate::menu::SPEC.id);
        assert!(SPEC.own_page && SPEC.config_path.is_none());
        assert!(OPTIONS.iter().all(|option| option.no_persist));
        assert_eq!(OPTIONS[OPTION_COUNT - 1].layout, RowLayout::Header);
        assert_eq!(OPTION_COUNT, 14);
    }

    #[test]
    fn stage_controls_are_independent_and_unavailable_hooks_never_accept_an_edit() {
        let table = ModeTable::new();
        let ready = stages::SUPPORTED_MASK;
        let before = table.settings();
        assert_eq!(current(&table, 1, 0), None);
        assert_eq!(request(&table, 1, 1, 0), RequestStatus::Unavailable);
        assert_eq!(table.settings(), before);
        assert_eq!(request(&table, 1, 1, ready), RequestStatus::Applied);
        assert_eq!(current(&table, 1, ready), Some(1));
        assert_eq!(table.all(), Some(FxMode::On));
        for (option, spec) in OPTIONS
            .iter()
            .enumerate()
            .skip(FIRST_STAGE)
            .take(stages::COUNT)
        {
            assert!(!spec.read_only);
            assert!(stages::supported(option - FIRST_STAGE));
            assert_eq!(current(&table, option, ready), Some(1));
            assert_eq!(request(&table, option, 0, ready), RequestStatus::Applied);
            assert_eq!(current(&table, option, ready), Some(0));
        }
        assert_eq!(current(&table, 1, ready), Some(0));
        assert_eq!(
            request(&table, FIRST_STAGE + 2, 1, ready),
            RequestStatus::Applied
        );
        assert_eq!(current(&table, 1, ready), None);
        let before = table.settings();
        for (option, choice) in [
            (0, 0),
            (1, 2),
            (FIRST_STAGE, 2),
            (usize::MAX, 0),
            (OPTION_COUNT - 1, 0),
        ] {
            assert_eq!(
                request(&table, option, choice, ready),
                RequestStatus::Rejected
            );
            assert_eq!(table.settings(), before);
        }
        assert_eq!(
            crate::menu::current(&table, 0, true),
            Some(0),
            "fighter Scope unchanged"
        );
    }

    #[test]
    fn partial_hook_failure_disables_only_its_rows_and_the_bulk_control() {
        for available in [stages::PARTICLE_MASK, stages::MODEL_MASK] {
            let table = ModeTable::new();
            let before = table.settings();
            assert_eq!(current(&table, 1, available), None);
            assert_eq!(request(&table, 1, 1, available), RequestStatus::Unavailable);
            assert_eq!(table.settings(), before);
            for index in 0..stages::COUNT {
                let option = FIRST_STAGE + index;
                if available & (1 << index) != 0 {
                    assert_eq!(
                        request(&table, option, 1, available),
                        RequestStatus::Applied
                    );
                    assert_eq!(current(&table, option, available), Some(1));
                } else {
                    assert_eq!(
                        request(&table, option, 1, available),
                        RequestStatus::Unavailable
                    );
                    assert_eq!(current(&table, option, available), None);
                    assert_eq!(table.stage_mode(index), Some(FxMode::On));
                }
            }
        }
    }
}
