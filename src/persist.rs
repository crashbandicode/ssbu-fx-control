//! When the plugin's own thread rewrites the INI.
//!
//! Online Deluxe persists single-row edits into the same file from its own
//! background thread (host persistence, `config_path`). Editing "All" changes
//! every row, which the host cannot write, so this plugin rewrites the whole
//! file. To keep the two writers apart, the plugin waits until the table has
//! been quiet for [`QUIET_MS`] (the host writes within about 500 ms of an
//! edit) and then writes the complete state, which also covers any host write
//! that raced it.

/// Quiet period after the last change before the plugin writes.
pub const QUIET_MS: u64 = 2_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Saver {
    saved: u64,
    seen: u64,
    changed_at_ms: u64,
}

impl Saver {
    /// `generation` is the table generation that is already on disk.
    pub const fn new(generation: u64) -> Self {
        Self {
            saved: generation,
            seen: generation,
            changed_at_ms: 0,
        }
    }

    /// One poll. Returns true when the caller should write now.
    pub fn poll(&mut self, generation: u64, now_ms: u64) -> bool {
        if generation != self.seen {
            self.seen = generation;
            self.changed_at_ms = now_ms;
        }
        self.seen != self.saved && now_ms.saturating_sub(self.changed_at_ms) >= QUIET_MS
    }

    /// The write of `generation` succeeded.
    pub fn saved(&mut self, generation: u64) {
        self.saved = generation;
    }

    /// The write failed: try again after another quiet period.
    pub fn failed(&mut self, now_ms: u64) {
        self.changed_at_ms = now_ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waits_for_quiet_then_writes_once() {
        let mut saver = Saver::new(5);
        assert!(!saver.poll(5, 0));
        assert!(!saver.poll(6, 100));
        assert!(!saver.poll(7, 600), "a new edit restarts the wait");
        assert!(!saver.poll(7, 2_500));
        assert!(saver.poll(7, 2_600));
        saver.saved(7);
        assert!(!saver.poll(7, 5_000));
        assert!(!saver.poll(8, 5_000));
        assert!(saver.poll(8, 7_000));
        saver.failed(7_000);
        assert!(!saver.poll(8, 8_000));
        assert!(saver.poll(8, 9_000));
    }
}
