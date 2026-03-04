use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub db: DbSettings,
    pub gateway: GatewaySettings,
    pub read: ReadSettings,
    pub debug: bool,
}

#[derive(Deserialize, Debug)]
pub struct DbSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub pass: String,
}

impl DbSettings{
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.pass,
            self.host,
            self.port,
            self.name
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct GatewaySettings {
    host: String,
    port: u16,
    protocol: Protocol,
    pub endpoints: GatewayEndpoints,
}

impl GatewaySettings {
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

#[derive(Deserialize, Debug)]
pub struct GatewayEndpoints {}

#[derive(Deserialize, Debug)]
pub struct ReadSettings {
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
}

impl ReadSettings {
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

#[derive(Deserialize, Debug, Display)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Http,
    Https,
}

pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `development` or `production`.",
                other
            )),
        }
    }
}

pub fn settings() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let settings_directory = base_path.join("settings");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.toml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            settings_directory.join(environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}