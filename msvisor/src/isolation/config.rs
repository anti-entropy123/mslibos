use std::{fs, io::BufReader, path::PathBuf};

use anyhow;
use log::debug;
use ms_hostcall::types::ServiceName;
use serde::{Deserialize, Serialize};

use std::io;

use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct IsolationConfig {
    pub services: Vec<(ServiceName, PathBuf)>,
    pub apps: Vec<(ServiceName, PathBuf)>,
}

impl IsolationConfig {
    pub fn to_file(&self, p: PathBuf) -> Result<(), io::Error> {
        fs::write(
            p,
            serde_json::to_string_pretty(self).expect("failed to pretty json string"),
        )?;
        Ok(())
    }

    pub fn from_file(p: PathBuf) -> Result<Self, anyhow::Error> {
        let p = if !p.is_file() {
            utils::ISOL_CONFIG_PATH.join(p)
        } else {
            p
        };

        debug!("config file path: {}", p.to_str().unwrap());
        let content = fs::File::open(p)?;
        let config = serde_json::from_reader(BufReader::new(content))?;

        Ok(config)
    }
}
