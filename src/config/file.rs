// Copyright 2017-2019 int08h LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::File;
use std::io::Read;
use std::time::Duration;
use yaml_rust::YamlLoader;

use crate::config::ServerConfig;
use crate::config::{DEFAULT_BATCH_SIZE, DEFAULT_STATUS_INTERVAL};
use crate::key::KmsProtection;
use crate::Error;

///
/// Read a Roughenough server configuration ([ServerConfig](trait.ServerConfig.html))
/// from a YAML file.
///
/// Example config:
///
/// ```yaml
/// interface: 127.0.0.1
/// port: 8686
/// seed: f61075c988feb9cb700a4a6a3291bfbc9cab11b9c9eca8c802468eb38a43d7d3
/// ```
///
pub struct FileConfig {
    port: u16,
    interface: String,
    seed: Vec<u8>,
    batch_size: u8,
    secondsoffset: u64,
    status_interval: Duration,
    kms_protection: KmsProtection,
    health_check_port: Option<u16>,
}

impl FileConfig {
    pub fn new(config_file: &str) -> Result<Self, Error> {
        let mut infile = File::open(config_file).expect("failed to open config file");

        let mut contents = String::new();
        infile
            .read_to_string(&mut contents)
            .expect("could not read config file");

        let cfg = YamlLoader::load_from_str(&contents).expect("could not parse config file");

        if cfg.len() != 1 {
            return Err(Error::InvalidConfiguration(
                "Empty or malformed config file".to_string(),
            ));
        }

        let mut config = FileConfig {
            port: 0,
            interface: "".to_string(),
            seed: Vec::new(),
            batch_size: DEFAULT_BATCH_SIZE,
            secondsoffset: 0,
            status_interval: DEFAULT_STATUS_INTERVAL,
            kms_protection: KmsProtection::Plaintext,
            health_check_port: None,
        };

        for (key, value) in cfg[0].as_hash().unwrap() {
            match key.as_str().unwrap() {
                "port" => config.port = value.as_i64().unwrap() as u16,
                "interface" => config.interface = value.as_str().unwrap().to_string(),
                "batch_size" => config.batch_size = value.as_i64().unwrap() as u8,
                "secondsoffset" => config.secondsoffset = value.as_i64().unwrap() as u64,
                "seed" => {
                    let val = value.as_str().unwrap().to_string();
                    config.seed = hex::decode(val)
                        .expect("seed value invalid; 'seed' should be 32 byte hex value");
                }
                "status_interval" => {
                    let val = value.as_i64().expect("status_interval value invalid");
                    config.status_interval = Duration::from_secs(val as u64)
                }
                "kms_protection" => {
                    let val =
                        value.as_str().unwrap().parse().unwrap_or_else(|_| {
                            panic!("invalid kms_protection value: {:?}", value)
                        });
                    config.kms_protection = val
                }
                "health_check_port" => {
                    let val = value.as_i64().unwrap() as u16;
                    config.health_check_port = Some(val);
                }
                unknown => {
                    return Err(Error::InvalidConfiguration(format!(
                        "unknown config key: {}",
                        unknown
                    )));
                }
            }
        }

        Ok(config)
    }
}

impl ServerConfig for FileConfig {
    fn interface(&self) -> &str {
        self.interface.as_ref()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn seed(&self) -> Vec<u8> {
        self.seed.clone()
    }

    fn batch_size(&self) -> u8 {
        self.batch_size
    }

    fn secondsoffset(&self) -> u64 {
        self.secondsoffset
    }
    fn status_interval(&self) -> Duration {
        self.status_interval
    }

    fn kms_protection(&self) -> &KmsProtection {
        &self.kms_protection
    }

    fn health_check_port(&self) -> Option<u16> {
        self.health_check_port
    }
}
