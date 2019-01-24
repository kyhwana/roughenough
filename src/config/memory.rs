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

use crate::config::ServerConfig;
use crate::config::{DEFAULT_BATCH_SIZE, DEFAULT_STATUS_INTERVAL,DEFAULT_SECONDSOFFSET};
use crate::key::KmsProtection;
use std::time::Duration;

use hex;

/// A purely in-memory Roughenough config for testing purposes.
///
/// This is useful for testing or fuzzing a server without the need to create additional files.
pub struct MemoryConfig {
    pub port: u16,
    pub interface: String,
    pub seed: Vec<u8>,
    pub batch_size: u8,
    pub secondsoffset: u64,
    pub status_interval: Duration,
    pub kms_protection: KmsProtection,
    pub health_check_port: Option<u16>,
}

impl MemoryConfig {
    pub fn new(port: u16) -> MemoryConfig {
        MemoryConfig {
            port,
            interface: "127.0.0.1".to_string(),
            seed: hex::decode("a32049da0ffde0ded92ce10a0230d35fe615ec8461c14986baa63fe3b3bac3db")
                .unwrap(),
            batch_size: DEFAULT_BATCH_SIZE,
            secondsoffset: DEFAULT_SECONDSOFFSET,
            status_interval: DEFAULT_STATUS_INTERVAL,
            kms_protection: KmsProtection::Plaintext,
            health_check_port: None,
        }
    }
}

impl ServerConfig for MemoryConfig {
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
