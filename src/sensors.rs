// src/sensors.rs

use crate::{measm::*, Snapshot};

const ALPHA_MODEL: f32 = 0.05;
const ALPHA_SENSOR: f32 = 0.4;
const GRAVITY_SI: f32 = 9.80665;

pub struct Properties {
	pub(crate) time: u32,
	pub(crate) odr: u32,
	pub(crate) prng: u32,
	pub(crate) noise: f32,
	pub(crate) idx: usize,
}

impl Properties {
	const fn new(odr: u32, noise: f32) -> Self {
		Self { time: 0, odr, prng: 0x112DECAF, noise, idx: 0 }
	}
}

pub struct Sensor<T> {
	pub(crate) prop: Properties,
	pub(crate) kind: T
}

impl<T: Default> Sensor<T> {
	pub fn new(odr: u32, noise: f32) -> Self {
		Self {
			prop: Properties::new(odr, noise),
			kind: T::default()
		}
	}
}

pub trait Evaluatable<T> {
	fn prime(&mut self, value: T);
	fn select(point: &Snapshot) -> T;
	fn reading(&self) -> T;
	fn step(&mut self, target: T, prop: &mut Properties, dt: f32);
}

#[derive(Default)]
pub struct Accl {
	model: Triple,
	accel: Triple,
	velocity: Triple,
	position: Triple
}

impl Evaluatable<Triple> for Accl {
	fn prime(&mut self, value: Triple) {
		self.model = value;
		self.accel = value;
	}

	fn select(point: &Snapshot) -> Triple {
		point.accl
	}
	
	fn reading(&self) -> Triple {
		self.accel
	}

	fn step(&mut self, target: Triple, prop: &mut Properties, dt: f32)
	{
		self.model = Triple::iir(self.model, target, ALPHA_MODEL);

		let net_accel = self.model.sub_scalar(GRAVITY_SI);
		self.velocity = self.velocity.add(net_accel.scale(dt));
		self.position = self.position.add(self.velocity.scale(dt));

		let noisy = self.model.noise(prop.noise, &mut prop.prng);
		self.accel = Triple::iir(self.accel, noisy, ALPHA_SENSOR);
	}
}

impl Accl {
	pub(crate) fn current_velocity(&self) -> Triple {
		self.velocity
	}
	pub(crate) fn current_position(&self) -> Triple {
		self.position
	}
}

#[derive(Default)]
pub struct Gyro {
	model: Triple,
	omega: Triple
}

impl Evaluatable<Triple> for Gyro {
	fn prime(&mut self, value: Triple) {
		self.model = value;
		self.omega = value;
	}

	fn select(point: &Snapshot) -> Triple {
		point.gyro
	}
	
	fn reading(&self) -> Triple {
		self.omega
	}

	fn step(&mut self, target: Triple, prop: &mut Properties, _dt: f32)
	{
		self.model = Triple::iir(self.model, target, ALPHA_MODEL);
		let noisy = self.model.noise(prop.noise, &mut prop.prng);
		self.omega = Triple::iir(self.omega, noisy, ALPHA_SENSOR);
	}
}