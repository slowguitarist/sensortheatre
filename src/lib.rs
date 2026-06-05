// src/lib.rs

#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
use panic_halt as _;

mod measm;
mod sensors;

pub mod device;
pub mod presets;

use crate::measm::XYZ;
use crate::device::Sensor;
use crate::sensors::{Accl, Gyro};

pub type Accelerometer<const N: usize> = Sensor<N, XYZ, Accl>;
pub type Gyroscope<const N: usize> = Sensor<N, XYZ, Gyro>;