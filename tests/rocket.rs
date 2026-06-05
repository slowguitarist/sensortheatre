// tests/rocket.rs

use sensortheatre::{Accelerometer, Gyroscope};

use std::thread;
use std::time::{Duration, SystemTime};

#[test]
fn small_rocket_ideal_flight() {
    let mut accel = Accelerometer::<8>::new(40, 0.8, 0.3, (0.0, 0.0, 9.81));
    accel
        // 1. Sample boost
        .then(3500, (0.0, 0.0, 27.5))
        // 2. Coast
        .then(5500, (0.0, 0.0, 0.0))
        // 3. Apogee pitch, increased accel shift
        .then(1200, (2.4, 0.0, 1.1))
        // 4. Descent
        .then(4000, (0.0, 0.0, 0.0))
        // 5. Parachute deployment, increased gyro shift
        .then(3000, (0.0, 0.0, 18.5))
        // 6. Parachute expansion
        .then(7500, (0.0, 0.0, 9.81))
        // 7. Soft crash into ground
        .then(1000, (0.0, 0.0, 13.2));

    let mut gyro = Gyroscope::<8>::new(60, 0.3, 0.4, (0.0, 0.0, 0.0));
    gyro
        // 1. Sample boost
        .then(3500, (0.1, 0.1, 0.3))
        // 2. Coast
        .then(5500, (0.02, 0.02, 0.05))
        // 3. Apogee pitch, increased accel shift
        .then(1200, (0.15, 0.05, 0.15))
        // 4. Descent
        .then(4000, (0.02, 0.02, 0.05))
        // 5. Parachute deployment, increased gyro shift
        .then(3000, (0.6, 0.6, 1.2))
        // 6. Parachute expansion
        .then(7500, (0.08, 0.08, 0.15))
        // 7. Soft crash into ground
        .then(1000, (0.2, 0.2, 0.4));

    let now = SystemTime::now();

    while let Ok(elapsed) = now.elapsed() {
        let ts = elapsed.as_millis() as u32;

        if elapsed >= Duration::from_secs(26) {
            break;
        }

        if let Ok(a) = accel.get(ts) {
            println!("ACCL {} | {:?}", ts, a);
        }

        if let Ok(g) = gyro.get(ts) {
            println!("GYRO {} | {:?}", ts, g);
        }

        println!("{} VEL {:?}", ts, accel.system_velocity(ts));
        println!("{} POS {:?}", ts, accel.system_position(ts));

        thread::sleep(Duration::from_millis(100));
    }
}