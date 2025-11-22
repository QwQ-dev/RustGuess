use crate::config::settings::Settings;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub fn get_settings_path_buf() -> PathBuf {
    PathBuf::from("config")
        .join("settings")
        .with_extension("yml")
}

pub static SETTINGS_CONTENT: Lazy<String> = Lazy::new(|| {
    let settings_path_buf = get_settings_path_buf();

    if let Some(parent_dir) = settings_path_buf.parent() {
        std::fs::create_dir_all(parent_dir).expect("Failed to create config dir");
    }

    if !std::fs::exists(&settings_path_buf).expect("Failed to check for existence") {
        "".to_string()
    } else {
        std::fs::read_to_string(&settings_path_buf).expect("Failed read settings.yaml")
    }
});

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    if SETTINGS_CONTENT.is_empty() {
        let new_settings = Settings::new();
        let new_settings_string =
            serde_yaml::to_string(&new_settings).expect("Failed serialize settings");
        std::fs::write(get_settings_path_buf(), new_settings_string)
            .expect("Failed write settings");

        return new_settings;
    }

    serde_yaml::from_str(&SETTINGS_CONTENT)
        .expect("Failed parse settings. Please delete settings from config file and re-create it.")
});
