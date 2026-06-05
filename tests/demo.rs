// tests/demo.rs

use core::{assert_eq, assert};

use sensortheatre::presets::IMU;

#[test]
fn short_ascent_demo() {
	// Initialize preset and write the plot!
	// Const parameter is the amount of data points (# then + 1)

	let mut imu = IMU::<4>::init(
		10,						// ODR (ms)
		0.5,					// Noise
		0.6,					// IIR smoothing factor 0 < k < 1
		(0.0, 0.0, 9.81),		// init accel
		(0.0, 0.0, 0.0)			// init gyro
	);

	imu
		.then(2000, (0.0, 0.0, 9.81), (0.0, 0.0, 0.0))
		.then(800, (0.7, 0.0, 25.0), (5.0, 0.0, 0.0))
		.then(3200, (0.2, -0.1, 9.81), (0.1, 0.2, 10.0));

	let mut time = 5; // Can be either hardware or set manually

	let no_progress = imu.poll(time);

	assert_eq!(no_progress, Err(((0.0, 0.0, 9.81), (0.0, 0.0, 0.0))));

	time += 2400;

	let progress_a = imu.accl.get(time);
	let progress_g = imu.gyro.get(time);

	assert!(progress_a.is_ok_and(|k| k != (0.0, 0.0, 9.81)));
	assert!(progress_g.is_ok_and(|k| k != (0.0, 0.0, 0.0)));
}