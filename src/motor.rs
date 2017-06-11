extern crate rust_pigpio;

use self::rust_pigpio::*;
use self::rust_pigpio::pwm::*;

use std;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;

use std::time::Duration;

const MAX_VALUE : u32 = 1975;
const MIN_VALUE : u32 = 1100;

pub struct MotorManager {
    motors: Vec<Motor>
}

impl MotorManager {

    pub fn new() -> MotorManager {
        let mm = MotorManager{ motors: Vec::new() };
        mm.initialize();
        mm
    }

    pub fn initialize(&self) {
        initialize().unwrap();
        println!("Initialized Motor Manager!");
    }

    pub fn terminate(&mut self) {
        for motor in &mut self.motors {
            motor.stop();
        }
        terminate();
        println!("Stopped.");
    }

    pub fn calibrate(&mut self) {
//        println!("Calibrating. This may take a moment");
//        let mut handles: Vec<JoinHandle<()>> = Vec::new();
//        for &mut motor in self.motors {
//            handles.push(motor.calibrate());
//        }
//
//        sleep(Duration::from_secs(10));
//        println!("Weird eh! Special tone eh? WHAT DID I SAID!!!");
//
//        for mut handle in handles {
//            handle.join();
//        }
//
//        println!("Done calibrating. Restarting.");
//        self.terminate();
//        sleep(Duration::from_secs(10));
//
//        self.initialize();
    }

    pub fn arm(&mut self) {
        println!("Arming motors.");

        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        for mut motor in &mut self.motors {
            handles.push(motor.arm());
        }

        for mut handle in handles {
            handle.join();
        }

        println!("Motors armed.");

        println!("Starting motors.");

        for mut motor in &mut self.motors {
            motor.set_power(MIN_VALUE);
        }
    }

    pub fn new_motor(&mut self, gpio_pin: u32) {
        //        if !initialized {
        //            println!("Error: Not initialized.");
        //            return;
        //        }
        //        if MotorManager::used_pins.contains(gpio_pin) {
        //            println!("Error: Pin in use.");
        //            return;
        //        }
        let motor = Motor::new(gpio_pin);
        self.motors.push(motor);
        //        used_pins.insert(gpio_pin);
    }

    pub fn set_power(&mut self, motor_num: usize, power: u32) {
        self.motors[motor_num].set_power(power);
    }
}

//impl std::ops::Drop for MotorManager {
//    fn drop(&mut self) {
//        self.terminate();
//    }
//}

pub struct Motor {
    pin: u32,
    current_power: u32
}

impl Motor {

    pub fn new(gpio_pin: u32) -> Motor {
        set_mode(gpio_pin, OUTPUT).unwrap();
        set_pwm_range(gpio_pin, 2000).unwrap();
        set_pwm_frequency(gpio_pin, 500).unwrap();
        Motor { pin: gpio_pin, current_power: 0 }
    }

    pub fn calibrate(&mut self) -> thread::JoinHandle<()> {
        let gpio = self.pin;
        thread::spawn(move || {
            servo(gpio, 0).unwrap();
            sleep(Duration::from_secs(4));
            servo(gpio, 2000).unwrap();
            sleep(Duration::from_secs(4));
            servo(gpio, 1000).unwrap();
            sleep(Duration::from_secs(8));
            write(gpio, OFF).unwrap();
            sleep(Duration::from_secs(8));
        })
    }

    pub fn arm(&mut self) -> thread::JoinHandle<()> {
        let gpio = self.pin;
        thread::spawn(move || {
            pwm(gpio, 1000).unwrap();
            sleep(Duration::from_secs(2));

            pwm(gpio, 1100).unwrap();
        })
    }

    pub fn set_power(&mut self, mut power: u32) {
        if power > MAX_VALUE {
            power = MAX_VALUE;
        }
        else if power < MIN_VALUE {
            power = MIN_VALUE;
        }

        pwm(self.pin, power).unwrap();
        self.current_power = power
    }

    #[allow(dead_code)]
    pub fn get_power(&self) -> u32 {
        self.current_power
    }

    pub fn stop(&mut self)  {
        write(self.pin, OFF).unwrap();
        self.current_power = 0;
    }
}

