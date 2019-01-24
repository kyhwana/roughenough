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

//!
//! CLI used to encrypt the Roughenough long-term key using one of the KMS implementations
//!

#[macro_use]
extern crate log;

use clap::{App, Arg};
use roughenough::roughenough_version;

#[cfg(feature = "awskms")]
fn aws_kms(kms_key: &str, plaintext_seed: &[u8]) {
    use roughenough::kms::{AwsKms, EnvelopeEncryption};

    let client = AwsKms::from_arn(kms_key).unwrap();

    match EnvelopeEncryption::encrypt_seed(&client, &plaintext_seed) {
        Ok(encrypted_blob) => {
            println!("kms_protection: \"{}\"", kms_key);
            println!("seed: {}", hex::encode(&encrypted_blob));
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}

#[cfg(feature = "gcpkms")]
fn gcp_kms(kms_key: &str, plaintext_seed: &[u8]) {
    use roughenough::kms::{EnvelopeEncryption, GcpKms};

    let client = GcpKms::from_resource_id(kms_key).unwrap();

    match EnvelopeEncryption::encrypt_seed(&client, &plaintext_seed) {
        Ok(encrypted_blob) => {
            println!("kms_protection: \"{}\"", kms_key);
            println!("seed: {}", hex::encode(&encrypted_blob));
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}

#[allow(unused_variables)]
pub fn main() {
    use log::Level;

    simple_logger::init_with_level(Level::Info).unwrap();

    let matches = App::new("roughenough-kms")
        .version(roughenough_version().as_ref())
        .long_about("Encrypt a Roughenough server's long-term seed using a KMS")
        .arg(
            Arg::with_name("KEY_ID")
                .short("k")
                .long("kms-key")
                .takes_value(true)
                .required(true)
                .help("Identity of the KMS key to be used"),
        ).arg(
            Arg::with_name("SEED")
                .short("s")
                .long("seed")
                .takes_value(true)
                .required(true)
                .help("32 byte hex seed for the server's long-term identity"),
        ).get_matches();

    let kms_key = matches.value_of("KEY_ID").unwrap();
    let plaintext_seed = matches
        .value_of("SEED")
        .map(|seed| hex::decode(seed).expect("Error parsing seed value"))
        .unwrap();

    if plaintext_seed.len() != 32 {
        error!(
            "Seed must be 32 bytes long; provided seed is {}",
            plaintext_seed.len()
        );
        return;
    }

    if cfg!(feature = "awskms") {
        #[cfg(feature = "awskms")]
        aws_kms(kms_key, &plaintext_seed);
    } else if cfg!(feature = "gcpkms") {
        #[cfg(feature = "gcpkms")]
        gcp_kms(kms_key, &plaintext_seed);
    } else {
        warn!("KMS support was not compiled, nothing to do.");
        warn!("For information on KMS support see the Roughenough documentation.");
    }
}
