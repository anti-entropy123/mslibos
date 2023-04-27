use std::{fs, io::BufReader, path::PathBuf};

use anyhow;
use ms_hostcall::types::ServiceName;
use serde::{Deserialize, Serialize};

use std::io;

#[derive(Serialize, Deserialize)]
pub struct IsolationConfig {
    pub services: Vec<(ServiceName, PathBuf)>,
    pub app: (ServiceName, PathBuf),
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
        let content = fs::File::open(p)?;
        let mut config = serde_json::from_reader(BufReader::new(content))?;

        Ok(config)
    }
}
