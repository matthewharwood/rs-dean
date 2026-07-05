use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Settings {
    #[garde(length(min = 1))]
    pub id: String,
    #[garde(skip)]
    pub theme: Theme,
    #[garde(skip)]
    pub reduced_motion: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: "settings".to_owned(),
            theme: Theme::Light,
            reduced_motion: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct Progress {
    #[garde(length(min = 1))]
    pub id: String,
    #[garde(range(min = 1))]
    pub level: u32,
    #[garde(skip)]
    pub completed: bool,
}

impl Progress {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            level: 1,
            completed: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Validate)]
pub struct AppSnapshot {
    #[garde(dive)]
    pub settings: Settings,
    #[garde(dive)]
    pub progress: Vec<Progress>,
}

impl Default for AppSnapshot {
    fn default() -> Self {
        Self {
            settings: Settings::default(),
            progress: vec![Progress::new("welcome")],
        }
    }
}

pub fn validate_snapshot(snapshot: &AppSnapshot) -> Result<(), garde::Report> {
    snapshot.validate()
}

#[cfg(test)]
mod tests {
    use super::{AppSnapshot, validate_snapshot};

    #[test]
    fn default_snapshot_validates() {
        assert!(validate_snapshot(&AppSnapshot::default()).is_ok());
    }
}
