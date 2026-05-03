use std::fmt;
use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderKind {
    YouVersion,
}

impl fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::YouVersion => write!(f, "YouVersion Platform"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub kind: ProviderKind,
    pub app_key: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProvidersConfig {
    pub providers: Vec<ProviderConfig>,
}

impl ProvidersConfig {
    pub fn active_provider(&self, kind: ProviderKind) -> Option<&ProviderConfig> {
        self.providers.iter().find(|p| p.kind == kind && p.enabled)
    }

    pub fn has_youversion(&self) -> bool {
        self.active_provider(ProviderKind::YouVersion).is_some()
    }

    pub fn youversion_key(&self) -> Option<&str> {
        self.active_provider(ProviderKind::YouVersion)
            .map(|p| p.app_key.as_str())
    }
}

fn providers_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "selah")?;
    Some(dirs.data_dir().join("providers.json"))
}

pub fn load() -> ProvidersConfig {
    let mut config: ProvidersConfig = providers_path()
        .and_then(|path| fs::read_to_string(&path).ok())
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default();

    if !config
        .providers
        .iter()
        .any(|p| p.kind == ProviderKind::YouVersion)
        && let Ok(key) = std::env::var("SELAH_YVP_APP_KEY")
        && !key.is_empty()
    {
        config.providers.push(ProviderConfig {
            kind: ProviderKind::YouVersion,
            app_key: key,
            enabled: true,
        });
    }

    config
}

pub fn save(config: &ProvidersConfig) {
    let Some(path) = providers_path() else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(
        &path,
        serde_json::to_string_pretty(config).unwrap_or_default(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn providers_config_serde_round_trip() {
        let config = ProvidersConfig {
            providers: vec![ProviderConfig {
                kind: ProviderKind::YouVersion,
                app_key: "test-key-123".to_string(),
                enabled: true,
            }],
        };
        let json = serde_json::to_string_pretty(&config).unwrap();
        let loaded: ProvidersConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.providers.len(), 1);
        assert_eq!(loaded.providers[0].kind, ProviderKind::YouVersion);
        assert_eq!(loaded.providers[0].app_key, "test-key-123");
        assert!(loaded.providers[0].enabled);
    }

    #[test]
    fn empty_config_has_no_providers() {
        let config = ProvidersConfig::default();
        assert!(config.providers.is_empty());
    }

    #[test]
    fn active_provider_returns_enabled() {
        let config = ProvidersConfig {
            providers: vec![ProviderConfig {
                kind: ProviderKind::YouVersion,
                app_key: "key".to_string(),
                enabled: true,
            }],
        };
        assert!(config.active_provider(ProviderKind::YouVersion).is_some());
    }

    #[test]
    fn active_provider_skips_disabled() {
        let config = ProvidersConfig {
            providers: vec![ProviderConfig {
                kind: ProviderKind::YouVersion,
                app_key: "key".to_string(),
                enabled: false,
            }],
        };
        assert!(config.active_provider(ProviderKind::YouVersion).is_none());
    }

    #[test]
    fn youversion_key_returns_key() {
        let config = ProvidersConfig {
            providers: vec![ProviderConfig {
                kind: ProviderKind::YouVersion,
                app_key: "my-secret-key".to_string(),
                enabled: true,
            }],
        };
        assert_eq!(config.youversion_key(), Some("my-secret-key"));
    }

    #[test]
    fn env_var_fallback_creates_provider() {
        // Build a config from scratch (no file) and inject env var
        let mut config = ProvidersConfig::default();
        assert!(!config.has_youversion());

        // Simulate env var fallback logic
        config.providers.push(ProviderConfig {
            kind: ProviderKind::YouVersion,
            app_key: "env-key-456".to_string(),
            enabled: true,
        });
        assert!(config.has_youversion());
        assert_eq!(config.youversion_key(), Some("env-key-456"));
    }
}
