#![allow(dead_code)]

use crate::errors::RockError;
use crate::profile::Profile;
use profile::buffer::ProfileDecoder;
use std::path::Path;

mod binutils;
mod driver;
mod errors;
mod http_server;
mod profile;
mod reports;

#[derive(Default)]
pub struct Options {
    profile_path: String,
}

fn pprof(op: &mut Options) {
    if op.profile_path.is_empty() {
        // this is warning, but ok
        eprintln!("binary path is empty")
    }

    // if wrong path provided -> panic
    if !Path::exists(op.profile_path.as_ref()) {
        panic!(format!("provided path {} does not exist", op.profile_path));
    }

    let profile = load_binary(&op.profile_path).unwrap();
    println!("{}", profile.to_string());
}

fn load_binary(path: &str) -> Result<Profile, RockError> {
    match std::fs::read(path) {
        Ok(data) => match profile::buffer::Buffer::unmarshal(data) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        },
        Err(err) => Err(RockError::Unknown {
            reason: err.to_string(),
        }),
    }
}

#[cfg(test)]
mod lib_tests {
    #[test]
    fn main_tests() {
        let mut op = super::Options::default();
        op.profile_path = "tests/CPU.pb.gz".to_string();
        super::pprof(&mut op);
    }
}
