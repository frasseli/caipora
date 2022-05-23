#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub node: NodeSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub secret: String,
    pub policy_vkey_file: String,
    pub policy_skey_file: String,
}

#[derive(serde::Deserialize)]
pub struct NodeSettings {
    pub environment: String,
    pub magic_number: String,
}

/// Main method to get the configurations for the app
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    // Add in settings from environment variables (with a prefix of APP and '__' as separator)
    // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}
