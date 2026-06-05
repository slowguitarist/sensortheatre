// src/sensors.rs

use crate::measm::{Noisy, Vector, XYZ};
use crate::device::Properties;

const SMOOTH_FAC: f32 = 0.16667;
const GRAVITY_SI: f32 = 9.80665;

pub trait SensorKind<T> {
	fn init(&mut self, value: T);
	fn reading(&self) -> T;
	fn step(&mut self, target: T, p: &mut Properties, dt: f32);
}

#[derive(Default)]
pub struct Accl {
	model: XYZ,
	accel: XYZ,
	velocity: XYZ,
	position: XYZ
}

impl SensorKind<XYZ> for Accl {
	fn init(&mut self, value: XYZ) {
		self.model = value;
		self.accel = value;
	}
	
	fn reading(&self) -> XYZ {
		self.accel
	}

	fn step(&mut self, target: XYZ, p: &mut Properties, dt: f32)
	{
		self.model = self.model.iir(target, SMOOTH_FAC);

		let grav = (0.0, 0.0, GRAVITY_SI);
		let net_accel = self.model.sub(grav);

		self.velocity = self.velocity.add(net_accel.scale(dt));
		self.position = self.position.add(self.velocity.scale(dt));

		let noisy = self.model.noise(p.noise, &mut p.prng);
		self.accel = self.accel.iir(noisy, p.alpha);
	}
}

impl Accl {
	pub(crate) fn current_velocity(&self) -> XYZ {
		self.velocity
	}
	pub(crate) fn current_position(&self) -> XYZ {
		self.position
	}
}

#[derive(Default)]
pub struct Gyro {
	model: XYZ,
	omega: XYZ
}

impl SensorKind<XYZ> for Gyro {
	fn init(&mut self, value: XYZ) {
		self.model = value;
		self.omega = value;
	}
	
	fn reading(&self) -> XYZ {
		self.omega
	}

	fn step(&mut self, target: XYZ, p: &mut Properties, _dt: f32)
	{
		self.model = self.model.iir(target, SMOOTH_FAC);
		let noisy = self.model.noise(p.noise, &mut p.prng);
		self.omega = self.omega.iir(noisy, p.alpha);
	}
}