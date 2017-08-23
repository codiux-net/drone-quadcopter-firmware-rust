extern crate serde;
extern crate serde_json;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::string::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct SensorCalibrations {
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32
}

impl SensorCalibrations {
    pub fn new() -> Self {
        let mut file = File::open("sensors.toml");
        if file.is_err() {
            return SensorCalibrations {
                gyro_x: 0.0,
                gyro_y: 0.0,
                gyro_z: 0.0,
                accel_x: 0.0,
                accel_y: 0.0,
                accel_z:  0.0
            }
        }
        let mut contents = String::new();
        if file.unwrap().read_to_string(&mut contents).expect("Failed to read sensors.toml") == 0 {
            return SensorCalibrations {
                gyro_x: 0.0,
                gyro_y: 0.0,
                gyro_z: 0.0,
                accel_x: 0.0,
                accel_y: 0.0,
                accel_z:  0.0
            }
        }
        toml::from_str(contents.as_ref()).unwrap()
    }

    pub fn write_calibration(&self) {
        println!("Writing to toml");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("sensors.toml").expect("Couldn't open or create new file");
        println!("Values to write: {:?}", self);
        let toml = toml::to_string(&self).unwrap();
        println!("{}", toml);
//        file.write(toml.as_bytes());
        file.write_all(toml.as_bytes());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub x_kp: f32,
    pub x_ki: f32,
    pub x_kd: f32,
    pub y_kp: f32,
    pub y_ki: f32,
    pub y_kd: f32,
    pub z_kp: f32,
    pub z_ki: f32,
    pub z_kd: f32,
    pub desired_angle: f32,
    pub motors: Vec<u32>,
    pub motor_cutoff: f32,
    pub motors_on: bool,
    pub integral_decay_time: f32,
    pub integral_bandwidth: f32,
    pub server_address: String,
    pub hover_power: u32,
    pub max_motor_speed: u32,
    pub debug_websocket_port: i32,
    pub sea_level_pressure: f32,
    pub derivative_sampling: f32,
    pub integral_decay: f32,
    pub sensors: Vec<String>,
    pub angle_offset_x: f32,
    pub angle_offset_y: f32,
    pub logging: bool,
    pub logging_freq: i32,
    pub real_time_debugging: bool,
    pub motor_frequency: u32,
    pub sensor_sample_frequency: u32,
    pub imu_frequency: u32,
    pub pid_frequency: u32
}

impl Config {
    pub fn new() -> Config {
        let mut file = File::open("config.json").expect("failed to open config.json");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("failed to read config.json");
        // Parse the string of data into serde_json::Value.
        serde_json::from_str(&contents).expect("failed to desrialize json")
    }
}
