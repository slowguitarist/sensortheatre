// src/presets.rs

use crate::{Accelerometer, Gyroscope, measm::XYZ};

pub struct IMU<const N: usize> {
	pub accl: Accelerometer<N>,
	pub gyro: Gyroscope<N>
}

impl<const N: usize> IMU<N> {
	pub fn init(odr: u32, noise: f32, alpha: f32, a: XYZ, g: XYZ) -> Self {
		Self {
			accl: Accelerometer::<N>::new(odr, noise, alpha, a),
			gyro: Gyroscope::<N>::new(odr, noise, alpha, g)
		}
	}

	pub fn then(&mut self, ts: u32, a: XYZ, g: XYZ) -> &mut Self {
		self.accl.then(ts, a);
		self.gyro.then(ts, g);
		self
	}

	pub fn poll(&mut self, ts: u32) -> Result<(XYZ, XYZ), (XYZ, XYZ)> {
		let acc = self.accl.get(ts);
		let gyr = self.gyro.get(ts);

		let (Ok(a) | Err(a)) = acc;
		let (Ok(g) | Err(g)) = gyr;

		if acc.is_ok() && gyr.is_ok() {
			Ok((a, g))
		} else {
			Err((a, g))
		}
	}
}