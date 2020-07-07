use serde::{Deserialize, Serialize};
use yes_or_no::yes_or_no;

yes_or_no!(NonInteractive);

impl NonInteractive {
    fn auto() -> Self {
        let is_ci = {
            let ci = std::env::var("CI").ok();
            ci.as_deref() == Some("true") || ci.as_deref() == Some("1")
        };
        if is_ci {
            log::info!(
                "env var `CI` is set to `true` or `1`; automatically running in non-interactive mode"
            );
            Self::Yes
        } else {
            Self::No
        }
    }

    pub fn from_flag(flag: bool) -> Self {
        if flag {
            Self::Yes
        } else {
            Self::auto()
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NoiseLevel {
    Polite,
    LoudAndProud,
    FranklyQuitePedantic,
}

impl Default for NoiseLevel {
    fn default() -> Self {
        Self::Polite
    }
}

impl NoiseLevel {
    pub fn from_occurrences(occurrences: u64) -> Self {
        match occurrences {
            0 => Self::Polite,
            1 => Self::LoudAndProud,
            _ => Self::FranklyQuitePedantic,
        }
    }

    pub fn polite(self) -> bool {
        matches!(self, Self::Polite)
    }

    pub fn loud(self) -> bool {
        matches!(self, Self::LoudAndProud)
    }

    pub fn pedantic(self) -> bool {
        matches!(self, Self::FranklyQuitePedantic)
    }
}

yes_or_no!(SkipDevTools);

yes_or_no!(ReinstallDeps);

yes_or_no!(OpenInEditor);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Profile {
    Debug,
    Release,
}

impl Profile {
    pub fn from_flag(flag: bool) -> Self {
        if flag {
            Self::Release
        } else {
            Self::Debug
        }
    }

    pub fn debug(self) -> bool {
        matches!(self, Self::Debug)
    }

    pub fn release(self) -> bool {
        matches!(self, Self::Release)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }
}
