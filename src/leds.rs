// SPDX-License-Identifier: MPL-2.0

//! Reads the keyboard lock indicators (Caps Lock, Num Lock, Scroll Lock) from
//! the sysfs LED class devices, e.g. `/sys/class/leds/input3::capslock`.
//!
//! This approach works without the `input` group: the `brightness` files are
//! world-readable and reflect the real lock state of every keyboard.

use std::fs;
use std::path::Path;

/// State of the keyboard lock indicators.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LedState {
    /// `Some(true)` when Caps Lock is on, `None` when no LED was found.
    pub(crate) caps: Option<bool>,
    /// `Some(true)` when Num Lock is on, `None` when no LED was found.
    pub(crate) num: Option<bool>,
    /// `Some(true)` when Scroll Lock is on, `None` when no LED was found.
    pub(crate) scroll: Option<bool>,
}

impl LedState {
    /// Reads the current state from `/sys/class/leds/`.
    pub(crate) fn read() -> Self {
        Self {
            caps: read_led("::capslock"),
            num: read_led("::numlock"),
            scroll: read_led("::scrolllock"),
        }
    }
}

/// Returns whether any keyboard LED matching `suffix` is on.
///
/// `None` means no matching LED was found on this system.
fn read_led(suffix: &str) -> Option<bool> {
    let leds_dir = Path::new("/sys/class/leds");
    let entries = fs::read_dir(leds_dir).ok()?;

    let mut found = false;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };

        if !name.ends_with(suffix) {
            continue;
        }

        let Ok(brightness) = fs::read_to_string(entry.path().join("brightness")) else {
            continue;
        };

        found = true;
        if brightness.trim() == "1" {
            return Some(true);
        }
    }

    found.then_some(false)
}
