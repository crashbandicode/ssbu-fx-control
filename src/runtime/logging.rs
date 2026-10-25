//! `install.log`, `events.log` and the plugin's background thread.
//!
//! The thread wakes every 500 ms: it flushes the bounded event ring to
//! `events.log` and, after the table has been quiet for two seconds, rewrites
//! `config.ini` when the table changed (see `crate::persist`). Game hooks never
//! touch a file.
use super::{record, Code, DATA_DIR, RECORDER, TABLE};
use crate::{build_info, config, persist::Saver, scope::OnlineSource};
use std::{
    ffi::CString,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

extern "C" {
    // Module-local symbols from the Skyline Rust linker script.
    static __text_start: u8;
    static __bss_end: u8;
}

const MAX_LOG_BYTES: u64 = 4 * 1024 * 1024;
/// Stack of the background thread; the platform default is 2 MiB.
const THREAD_STACK_BYTES: usize = 128 * 1024;
static PANICKING: AtomicBool = AtomicBool::new(false);

fn config_path() -> std::path::PathBuf {
    Path::new(DATA_DIR).join("config.ini")
}

/// Persist startup progress before anything else can fail.
pub(super) fn prepare() {
    let root = Path::new(DATA_DIR);
    if fs::create_dir_all(root).is_ok() {
        if let Ok(mut file) = rotate(&root.join("install.log")) {
            let _ = writeln!(file, "build={}\nphase=entered_main", build_info::BUILD_ID);
            let _ = file.sync_all();
        }
    }
}

pub(super) fn startup_phase(phase: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(DATA_DIR).join("install.log"))
    {
        let _ = writeln!(file, "phase={phase}");
        let _ = file.sync_all();
    }
}

/// Synchronous only during installation, never in a frame or effect callback.
pub(super) fn note_install(lines: &[String]) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(DATA_DIR).join("install.log"))
    {
        for line in lines {
            let _ = writeln!(file, "{line}");
        }
        let _ = file.sync_all();
    }
}

/// Synchronous only during installation, never in a frame or effect callback.
pub(super) fn install_checkpoint(code: Code, address: u64, detail: u64) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(DATA_DIR).join("install.log"))
    {
        let _ = writeln!(file, "{} 0x{address:x} 0x{detail:x}", code as u16);
        let _ = file.sync_all();
    }
}

/// Changed nonzero model counters. A 5 s wake scans; later 500 ms wakes drain
/// the remainder, 32 samples at a time, so one scan cannot fill the event ring.
fn publish_stage_counts(iteration: u64) {
    if !crate::stage_diag::DIAG.enabled() {
        return;
    }
    let scan = iteration.is_multiple_of(10);
    for sample in crate::stage_diag::DIAG.drain_model_counts(crate::stage_diag::PUBLISH_BATCH, scan)
    {
        record(Code::FxStageModelCount, sample.argument_a(), sample.count);
    }
}

fn rotate(path: &Path) -> io::Result<File> {
    if path.exists() {
        let previous = path.with_extension("previous.log");
        if previous.exists() {
            fs::remove_file(&previous)?;
        }
        fs::rename(path, previous)?;
    }
    OpenOptions::new().create(true).append(true).open(path)
}

/// skyline's std maps `GetEntryType`'s 0x202 to `ErrorKind::Other`. Ask the
/// native API so only a confirmed-missing file is created.
fn file_is_missing(path: &Path) -> bool {
    let Ok(path) = CString::new(path.as_os_str().as_encoded_bytes()) else {
        return false;
    };
    let mut entry_type = 0;
    unsafe { skyline::nn::fs::GetEntryType(&mut entry_type, path.as_ptr().cast()) == 0x202 }
}

fn read_existing(path: &Path) -> io::Result<String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(error) if error.kind() == io::ErrorKind::NotFound || file_is_missing(path) => {
            Ok(String::new())
        }
        Err(error) => Err(error),
    }
}

/// Rewrites the INI from the table, keeping unknown lines.
fn save_config() -> io::Result<()> {
    let path = config_path();
    crate::config_store::recover(&path)?;
    let existing = read_existing(&path)?;
    let text = config::render(&existing, &TABLE.settings());
    if text == existing {
        return Ok(());
    }
    crate::config_store::save(&path, &text)
}

/// Loads the INI into the table. A missing file is created with the defaults;
/// an unreadable one is left untouched and the defaults are used.
fn load_config() -> String {
    let path = config_path();
    if let Err(error) = crate::config_store::recover(&path) {
        return format!("config recovery failed: {error}");
    }
    match fs::read_to_string(&path) {
        Ok(text) => {
            TABLE.load(&config::parse(&text));
            "loaded config".to_owned()
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound || file_is_missing(&path) => {
            TABLE.load(&config::Settings::default());
            match save_config() {
                Ok(()) => "created default config".to_owned(),
                Err(error) => format!("default config not persisted: {error}"),
            }
        }
        Err(error) => {
            TABLE.load(&config::Settings::default());
            format!("config unreadable (file preserved, defaults used): {error}")
        }
    }
}

pub(super) fn start(version: &str, source: OnlineSource) {
    let root = Path::new(DATA_DIR);
    let dir_result = fs::create_dir_all(root);
    startup_phase("config_read_begin");
    let config_note = load_config();
    startup_phase("config_read_complete");
    let settings = TABLE.settings();
    record(
        Code::ConfigLoaded,
        source as u64,
        settings.all().map_or(u64::MAX, |mode| mode as u64),
    );
    let text_base =
        unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 };
    let plugin_start = core::ptr::addr_of!(__text_start) as u64;
    let plugin_end = core::ptr::addr_of!(__bss_end) as u64;
    let module_id = if plugin_end.saturating_sub(plugin_start) >= 0x60
        && unsafe { super::memory::accessible(plugin_start as usize, 0x60, false) }
    {
        let bytes = unsafe { std::slice::from_raw_parts(plugin_start as *const u8, 0x60) };
        if &bytes[0x10..0x14] == b"NRO0" {
            bytes[0x40..0x60]
                .iter()
                .map(|v| format!("{v:02x}"))
                .collect::<String>()
        } else {
            "header-unavailable".to_owned()
        }
    } else {
        "range-unavailable".to_owned()
    };
    let header = format!(
        "ssbu-fx-control {}\ncommit={}\nSmash={version}\ngame_text=0x{text_base:x}\nplugin_start=0x{plugin_start:x} plugin_end=0x{plugin_end:x}\nmodule_id={module_id}\nonline_source={source:?}\n{config_note}\nsettings={settings:?}\nsequence tick code arg0 arg1\n",
        build_info::BUILD_ID,
        build_info::GIT_COMMIT,
    );
    skyline::println!("{}", header);
    if let Err(error) = dir_result {
        skyline::println!("[ssbu-fx-control] data directory unavailable: {}", error);
    }
    if let Ok(mut file) = OpenOptions::new()
        .append(true)
        .open(root.join("install.log"))
    {
        let _ = file
            .write_all(header.as_bytes())
            .and_then(|_| file.sync_all());
    }
    let log_path = root.join("events.log");
    let mut log = match rotate(&log_path) {
        Ok(mut file) => {
            let _ = file
                .write_all(header.as_bytes())
                .and_then(|_| file.sync_data());
            Some(file)
        }
        Err(error) => {
            skyline::println!("[ssbu-fx-control] file logging unavailable: {}", error);
            None
        }
    };

    // Preserve other mods' panic hook. Hardware exceptions never reach this.
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !PANICKING.swap(true, Ordering::AcqRel) {
            if let Ok(mut file) = File::create(Path::new(DATA_DIR).join("panic.log")) {
                let _ = writeln!(file, "build={}\n{info}", build_info::BUILD_ID);
                if let Some(snapshot) = RECORDER.snapshot() {
                    for event in snapshot.after(0) {
                        let _ = writeln!(file, "{event:?}");
                    }
                    let _ = writeln!(file, "dropped={}", snapshot.dropped);
                }
                let _ = file.sync_all();
            }
        }
        previous(info);
    }));
    record(Code::Boot, plugin_start, text_base);
    record(Code::FxOnlineContext, source as u64, 0);

    let started = Instant::now();
    let mut saver = Saver::new(TABLE.generation());
    startup_phase("thread_begin");
    if let Err(error) = std::thread::Builder::new()
        .name("ssbu-fx-control".into())
        .stack_size(THREAD_STACK_BYTES)
        .spawn(move || {
            let mut persisted = 0;
            let mut dropped = 0;
            let mut iteration = 0u64;
            let mut error_reported = false;
            loop {
                std::thread::sleep(Duration::from_millis(500));
                iteration += 1;
                if iteration.is_multiple_of(10) {
                    record(Code::Heartbeat, TABLE.generation(), iteration);
                }
                publish_stage_counts(iteration);
                let now_ms = started.elapsed().as_millis() as u64;
                let generation = TABLE.generation();
                if saver.poll(generation, now_ms) {
                    match save_config() {
                        Ok(()) => {
                            saver.saved(generation);
                            record(Code::ConfigSaved, generation, 1);
                        }
                        Err(error) => {
                            saver.failed(now_ms);
                            record(Code::ConfigSaved, generation, 0);
                            if !error_reported {
                                skyline::println!(
                                    "[ssbu-fx-control] config save failed: {}",
                                    error
                                );
                                error_reported = true;
                            }
                        }
                    }
                }
                let Some(file) = log.as_mut() else {
                    continue;
                };
                if let Some(snapshot) = RECORDER.snapshot() {
                    let result: io::Result<()> = (|| {
                        if snapshot.newest.saturating_sub(persisted)
                            > crate::diagnostics::CAPACITY as u64
                        {
                            writeln!(file, "gap: ring overwritten after sequence {persisted}")?;
                        }
                        for event in snapshot.after(persisted) {
                            writeln!(
                                file,
                                "{} {} {} 0x{:x} 0x{:x}",
                                event.sequence, event.tick, event.code, event.a, event.b
                            )?;
                        }
                        if snapshot.dropped != dropped {
                            writeln!(file, "dropped_total={}", snapshot.dropped)?;
                        }
                        file.sync_data()
                    })();
                    match result {
                        Ok(()) => {
                            persisted = snapshot.newest;
                            dropped = snapshot.dropped;
                        }
                        Err(error) if !error_reported => {
                            skyline::println!("[ssbu-fx-control] log sink error: {}", error);
                            error_reported = true;
                        }
                        Err(_) => {}
                    }
                }
                if file
                    .metadata()
                    .map(|m| m.len() >= MAX_LOG_BYTES)
                    .unwrap_or(false)
                {
                    log = rotate(&log_path).ok();
                    if let Some(file) = log.as_mut() {
                        let _ = file.write_all(header.as_bytes());
                    }
                }
            }
        })
    {
        skyline::println!("[ssbu-fx-control] background thread failed: {}", error);
    }
    startup_phase("thread_returned");
}
