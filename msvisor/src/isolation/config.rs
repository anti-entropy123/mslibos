use std::{fs, io::BufReader, path::PathBuf};

use anyhow;
use ms_hostcall::types::ServiceName;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::io;

#[derive(Serialize, Deserialize)]
pub struct IsolationConfig {
    pub services: Vec<(ServiceName, PathBuf)>,
    pub app: (ServiceName, PathBuf),
}

impl IsolationConfig {
    pub fn to_file(&self, p: PathBuf) -> Result<(), io::Error> {
        let result = fs::write(p, json!(self).to_string())?;
        Ok(result)
    }

    pub fn from_file(p: PathBuf) -> Result<Self, anyhow::Error> {
        let content = fs::File::open(p)?;
        Ok(serde_json::from_reader(BufReader::new(content))?)
    }
}
