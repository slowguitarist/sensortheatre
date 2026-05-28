// src/lib.rs

#![no_std]

use panic_halt as _;
use crate::{measm::*, scenario::Scenario, utils::lcg_normalized};

mod utils;
mod measm;

pub mod scenario;

pub struct Simulation<const N: usize> {
	scenario: Scenario<N>,

	eval_time: u32,
	prng: u32,

	model_accl: Triple,
	model_gyro: Triple,

	velocity: Triple,
	position: Triple,

	raw_accl: Triple,
	raw_gyro: Triple,
	raw_baro: f32,

	shared_odr: u32,
	noise: SensorNoise,
}

impl<const N: usize> Simulation<N> {
	const GRAVITY: f32 = 9.80665;
	const ALPHA_MODEL: f32 = 0.05;
	const ALPHA_SENSOR: f32 = 0.4;

	pub fn new(scenario: Scenario<N>, start: u32, odr: u32, noiz: SensorNoise) -> Self {
		let init = scenario.points[0];
		Self {
			scenario,

			eval_time: start,
			prng: 0x5ED5C0DE,

			model_accl: init.accl,
			model_gyro: init.gyro,

			velocity: Triple::new(0.0, 0.0, 0.0),
			position: Triple::new(0.0, 0.0, 0.0),

			raw_accl: init.accl,
			raw_gyro: init.gyro,
			raw_baro: 0.0,

			shared_odr: odr,
			noise: noiz
		}
	}

	fn update(&mut self, time: u32) -> bool {
		let mut elapsed = time.wrapping_sub(self.eval_time);
		let dt = self.shared_odr as f32 / 1000.0;

		if elapsed < self.shared_odr {
			return false;
		}

		while elapsed >= self.shared_odr {
			self.eval_time = self.eval_time.wrapping_add(self.shared_odr);
			elapsed -= self.shared_odr;

			let (t_accl, t_gyro) = self.scenario.target(self.eval_time);

			self.model_accl = Triple::iir(self.model_accl, t_accl, Self::ALPHA_MODEL);
			self.model_gyro = Triple::iir(self.model_gyro, t_gyro, Self::ALPHA_MODEL);

			let net_accl = self.model_accl.sub_scalar(Self::GRAVITY);
			self.velocity = self.velocity.add(net_accl.scale(dt));
			self.position = self.position.add(self.velocity.scale(dt));

			let noisy_accl = self.model_accl.noise(self.noise.accl, &mut self.prng);
			let noisy_gyro = self.model_gyro.noise(self.noise.gyro, &mut self.prng);
			let noisy_baro = self.position.z + lcg_normalized(&mut self.prng) * self.noise.baro;

			self.raw_accl = Triple::iir(self.raw_accl, noisy_accl, Self::ALPHA_SENSOR);
			self.raw_gyro = Triple::iir(self.raw_gyro, noisy_gyro, Self::ALPHA_SENSOR);
			self.raw_baro = self.raw_baro + Self::ALPHA_SENSOR * (noisy_baro - self.raw_baro);
		}

		true
	}

	pub fn get_accl(&mut self, time: u32) -> Result<Triple, Triple> {
		if self.update(time) {
			Ok(self.raw_accl)
		} else {
			Err(self.raw_accl)
		}
	}

	pub fn get_gyro(&mut self, time: u32) -> Result<Triple, Triple> {
		if self.update(time) {
			Ok(self.raw_gyro)
		} else {
			Err(self.raw_gyro)
		}
	}

	pub fn get_baro(&mut self, time: u32) -> Result<f32, f32> {
		if self.update(time) {
			Ok(self.raw_baro)
		} else {
			Err(self.raw_baro)
		}
	}
}