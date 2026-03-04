use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Queries {
    pub read: Read,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    pub get_animal_by_id: String,
}

pub fn queries() -> Result<Queries, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let settings_directory = base_path.join("settings");

    let queries = config::Config::builder()
        .add_source(config::File::from(
            settings_directory.join("queries.yaml"),
        ))
        .build()?;

    queries.try_deserialize::<Queries>()
}