// tests/rocket.rs

use sensortheatre::{Simulation, Accelerometer, Gyroscope};
use sensortheatre::measm::Triple;

use std::thread;
use std::time::{Duration, SystemTime};

#[test]
fn small_rocket_ideal_flight() {
	let mut accel = Accelerometer::new(40, 0.8, 0.5);
	let mut gyro = Gyroscope::new(40, 0.3, 0.4);

	let mut flight = Simulation::<13>::init(
		Triple::new(0.0, 0.0, 9.81),
		Triple::new(0.0, 0.0, 0.0)
	);

	flight.add(&mut accel).add(&mut gyro)
	
		// 1. Sample boost
		.then(3500,
			Triple::new(0.0, 0.0, 27.5),
			Triple::new(0.1, 0.1, 0.3)
		)

		// 2. Coast
		.then(5500,
			Triple::new(0.0, 0.0, 0.0),
			Triple::new(0.02, 0.02, 0.05)
		)

		// 3. Apogee pitch, increased accel shift
		.then(1200,
			Triple::new(2.4, 0.0, 1.1),
			Triple::new(0.15, 0.05, 0.15)
		)

		// 4. Descent
		.then(4000,
			Triple::new(0.0, 0.0, 0.0),
			Triple::new(0.02, 0.02, 0.05)
		)

		// 5. Parachute deployment, increased gyro shift
		.then(3000,
			Triple::new(0.0, 0.0, 18.5),
			Triple::new(0.6, 0.6, 1.2)
		)

		// 6. Parachute expansion
		.then(7500,
			Triple::new(0.0, 0.0, 9.81),
			Triple::new(0.08, 0.08, 0.15)
		)

		// 7. Soft crash into ground
		.then(1000,
			Triple::new(0.0, 0.0, 13.2),
			Triple::new(0.2, 0.2, 0.4)
		);

	let now = SystemTime::now();

	while let Ok(elapsed) = now.elapsed() {
		let ts = elapsed.as_millis() as u32;

		if elapsed >= Duration::from_secs(26) {
			break;
		}

		if let Ok(a) = flight.get(&mut accel, ts) {
			println!("ACCL {} | {:?}", ts, a);
		}

		if let Ok(g) = flight.get(&mut gyro, ts) {
			println!("GYRO {} | {:?}", ts, g);
		}

		println!("{} VEL {:?}", ts, flight.vel(&mut accel, ts));
		println!("{} POS {:?}", ts, flight.pos(&mut accel, ts));

		thread::sleep(Duration::from_millis(100));
	}
}