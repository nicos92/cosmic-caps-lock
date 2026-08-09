// SPDX-License-Identifier: MPL-2.0

use cosmic::cosmic_config::cosmic_config_derive::CosmicConfigEntry;
use cosmic::cosmic_config::{self, CosmicConfigEntry};

pub(crate) const APP_ID: &str = "io.github.nicos92.cosmic-caps-lock";

const CONFIG_VERSION: u64 = 2;

/// Configuration that persists between application runs.
#[derive(Debug, Clone, CosmicConfigEntry)]
pub(crate) struct Config {
    /// Whether to show the Caps Lock indicator in the panel.
    pub(crate) show_caps: bool,
    /// Whether to show the Num Lock indicator in the panel.
    pub(crate) show_num: bool,
    /// Whether to show the Scroll Lock indicator in the panel.
    pub(crate) show_scroll: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_caps: true,
            show_num: true,
            show_scroll: true,
        }
    }
}

impl Config {
    fn config_handler() -> Option<cosmic_config::Config> {
        cosmic_config::Config::new(APP_ID, CONFIG_VERSION).ok()
    }

    fn load() -> Config {
        match Self::config_handler() {
            Some(handler) => Self::get_entry(&handler)
                .map_err(|(errors, _)| {
                    for why in errors {
                        eprintln!("error loading app config: {why}");
                    }
                })
                .unwrap_or_default(),
            None => Config::default(),
        }
    }
}

/// Flags passed to the application at startup.
#[derive(Debug, Clone)]
pub(crate) struct Flags {
    pub(crate) config: Config,
    pub(crate) config_handler: Option<cosmic_config::Config>,
}

impl Flags {
    pub(crate) fn new() -> Self {
        Self {
            config: Config::load(),
            config_handler: Config::config_handler(),
        }
    }
}
