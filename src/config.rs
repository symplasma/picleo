use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Default height for the picker interface
    pub height: Option<u16>,

    /// Enable mouse support by default
    pub mouse_enabled: Option<bool>,

    /// Default case matching behavior
    pub case_matching: Option<String>,

    /// Default normalization behavior
    pub normalization: Option<String>,

    /// Wrap around when navigating past first/last item
    pub wrap_around: Option<bool>,

    /// Invert mouse scroll direction
    pub invert_scroll: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            height: None,
            mouse_enabled: Some(true),
            case_matching: Some("smart".to_string()),
            normalization: Some("smart".to_string()),
            wrap_around: Some(true),
            invert_scroll: Some(false),
        }
    }
}

impl Config {
    /// Load configuration from platform-appropriate config directories
    pub fn load() -> Result<Self, figment::Error> {
        let mut figment = Figment::new();

        // Add config file paths in order of precedence (lowest to highest)
        if let Some(config_dir) = directories::ProjectDirs::from("", "", "picleo") {
            // System config
            if let Some(config_dir) = config_dir.config_dir().parent() {
                let system_config = config_dir.join("picleo").join("config");
                figment = Self::add_config_files(figment, &system_config);
            }

            // User config
            let user_config = config_dir.config_dir().join("config");
            figment = Self::add_config_files(figment, &user_config);
        }

        // Local config (highest precedence)
        figment = Self::add_config_files(figment, &PathBuf::from(".picleo"));

        // Environment variables (highest precedence)
        figment = figment.merge(Env::prefixed("PICLEO_"));

        figment.extract()
    }

    fn add_config_files(mut figment: Figment, base_path: &PathBuf) -> Figment {
        // Try different config file formats
        for extension in &["toml", "yaml", "yml", "json"] {
            let config_file = base_path.with_extension(extension);
            if config_file.exists() {
                figment = match extension {
                    &"toml" => figment.merge(Toml::file(config_file)),
                    &"yaml" | &"yml" => figment.merge(Yaml::file(config_file)),
                    &"json" => figment.merge(Json::file(config_file)),
                    _ => figment,
                };
            }
        }
        figment
    }

    /// Get the height setting, falling back to default if not configured
    pub fn height(&self) -> Option<u16> {
        self.height
    }

    /// Get the mouse enabled setting, falling back to default if not configured
    pub fn mouse_enabled(&self) -> bool {
        self.mouse_enabled.unwrap_or(true)
    }

    /// Get the wrap around setting, falling back to default if not configured
    pub fn wrap_around(&self) -> bool {
        self.wrap_around.unwrap_or(true)
    }

    /// Get the invert scroll setting, falling back to default if not configured
    pub fn invert_scroll(&self) -> bool {
        self.invert_scroll.unwrap_or(false)
    }
}
