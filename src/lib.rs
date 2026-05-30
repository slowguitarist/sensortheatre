// src/lib.rs

#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
use panic_halt as _;

mod utils;
pub mod measm;
pub mod sensors;

use crate::{measm::Triple, sensors::{Accl, Evaluatable, Gyro, Sensor}};

pub type Accelerometer = Sensor<Accl>;
pub type Gyroscope = Sensor<Gyro>;

#[derive(Clone, Copy, Default)]
pub struct Snapshot {
	pub(crate) time: u32,
	pub(crate) accl: Triple,
	pub(crate) gyro: Triple
}

pub struct Simulation<const N: usize> {
	pub(crate) points: [Snapshot; N],
	pub(crate) count: usize
}

impl<const N: usize> Simulation<N> {
	pub fn init(accl: Triple, gyro: Triple) -> Self {
		let mut k = Self {
			points: [Snapshot::default(); N],
			count: 1
		};
		k.points[0] = Snapshot { time: 0, accl, gyro };
		k
	}

	pub fn add<T, V>(&mut self, dev: &mut Sensor<T>) -> &mut Self
	where
		T: Evaluatable<V>,
		V: Copy
	{
		let value = T::select(&self.points[0]);
		dev.kind.prime(value);
		dev.prop.time = self.points[0].time;
		self
	}

	pub fn then(&mut self, ms: u32, accl: Triple, gyro: Triple) -> &mut Self {
		if self.count < N {
			let prev = self.points[self.count - 1].time;
			self.points[self.count] = Snapshot {
				time: prev + ms, accl, gyro
			};
			self.count += 1;
		}
		self
	}

	fn target<F, T>(&self, elapsed: u32, idx: &mut usize, select: F) -> T
	where 
		F: FnOnce(&Snapshot) -> T
	{
		while *idx < self.count - 1 && elapsed > self.points[*idx].time {
			*idx += 1;
		}

		select(&self.points[*idx])
	}

	pub fn get<T, V>(&self, dev: &mut Sensor<T>, time: u32) -> Result<V, V>
	where
		T: Evaluatable<V>,
		V: Copy
	{
		if time.wrapping_sub(dev.prop.time) < dev.prop.odr {
			return Err(dev.kind.reading())
		}

		let dt_secs = dev.prop.odr as f32 / 1000.0;

		while time.wrapping_sub(dev.prop.time) >= dev.prop.odr {
			dev.prop.time = dev.prop.time.wrapping_add(dev.prop.odr);
			dev.kind.step(
				self.target(dev.prop.time, &mut dev.prop.idx, T::select),
				&mut dev.prop,
				dt_secs
			);
		}

		Ok(dev.kind.reading())
	}

	pub fn vel(&self, dev: &mut Accelerometer, time: u32) -> Triple {
		let _ = self.get(dev, time);
		dev.kind.current_velocity()
	}

	pub fn pos(&self, dev: &mut Accelerometer, time: u32) -> Triple {
		let _ = self.get(dev, time);
		dev.kind.current_position()
	}
}