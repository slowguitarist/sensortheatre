use core::{assert_eq, assert};

use sensortheatre::{Simulation, Accelerometer, Gyroscope};
use sensortheatre::measm::Triple;

#[test]
fn rocket_demo() {
	// Initialize sensors and simulation
	let mut accel = Accelerometer::new(10, 0.8);
	let mut gyro = Gyroscope::new(15, 0.7);

	let mut ascent = Simulation::<4>::init(
		Triple::new(0.0, 0.0, 9.81),
		Triple::new(0.0, 0.0, 0.0)
	);
	
	// Write the plot!
	ascent.add(&mut accel).add(&mut gyro)
		.then(30_000,
			Triple::new(0.0, 0.0, 9.81),
			Triple::new(0.0, 0.0, 0.0)
		)
		.then(800,
			Triple::new(0.7, 0.0, 60.0),
			Triple::new(5.0, 0.0, 0.0)
		)
		.then(3700,
			Triple::new(0.2, -0.1, 9.81),
			Triple::new(0.1, 0.2, 10.0)
		);

	let mut time = 5; // Can be either hardware or set manually

	let mut a = ascent.get(&mut accel, time);
	let mut g = ascent.get(&mut gyro, time);

	assert_eq!(a, Err(Triple::new(0.0, 0.0, 9.81)));
	assert_eq!(g, Err(Triple::new(0.0, 0.0, 0.0)));

	time += 30_150;

	a = ascent.get(&mut accel, time);
	g = ascent.get(&mut gyro, time);

	assert!(a.is_ok_and(|k| k != Triple::new(0.0, 0.0, 9.81)));
	assert!(g.is_ok_and(|k| k != Triple::new(0.0, 0.0, 0.0)));
}