extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod calibrations;

pub type Config = config::Config;
pub type Calibrations = calibrations::Calibrations;
