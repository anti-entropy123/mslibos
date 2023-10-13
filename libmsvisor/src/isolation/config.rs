use std::{fs, io::BufReader, path::PathBuf};

use anyhow;
use log::debug;
use ms_hostcall::types::ServiceName;
use serde::{Deserialize, Serialize};

use std::io;

use crate::utils;

#[derive(Serialize, Deserialize, Clone)]
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

        #[cfg(feature = "namespace")]
        let mut config: IsolationConfig;
        #[cfg(not(feature = "namespace"))]
        let config: IsolationConfig;

        config = serde_json::from_reader(BufReader::new(content))?;

        #[cfg(feature = "namespace")]
        config.services.insert(
            0,
            (
                "libc".to_owned(),
                PathBuf::from("/lib/x86_64-linux-gnu/libc.so.6"),
            ),
        );

        Ok(config)
    }

    pub fn all_modules(&self) -> Vec<&(String, PathBuf)> {
        self.services.iter().chain(self.apps.iter()).collect()
    }
}

#[cfg(feature = "namespace")]
#[test]
fn all_modules_test() {
    let config = IsolationConfig::from_file(utils::ISOL_CONFIG_PATH.join("base_config.json"))
        .expect("load config failed");

    assert!(
        config
            .all_modules()
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<String>>()
            .contains(&"libc".to_owned()),
        "{:?}",
        config.all_modules()
    )
}
