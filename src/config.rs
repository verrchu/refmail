use std::{env::args, fs::OpenOptions, path::Path};

use crate::{
    types::{Referee, Referrer},
    Sections,
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub referrer: Referrer,
    pub referee: Referee,

    pub sections: Sections,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = args().nth(1).context("config path not provided")?;
        let config_file = OpenOptions::new()
            .read(true)
            .open(config_path)
            .context("failed to open config file")?;

        serde_yaml::from_reader(config_file).context("failed to parse config file")
    }

    pub fn load_from(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let config_file = OpenOptions::new()
            .read(true)
            .open(path.as_ref())
            .context("failed to open config file")?;

        serde_yaml::from_reader(config_file).context("failed to parse config file")
    }
}
