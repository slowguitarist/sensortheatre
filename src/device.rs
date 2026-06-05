// src/device.rs

use crate::measm::XYZ;
use crate::sensors::{SensorKind, Accl};

pub struct Properties {
	pub(crate) prng: u32,
	pub(crate) noise: f32,
	pub(crate) alpha: f32,
	time: u32,
	odr: u32,
	idx: usize,
}

impl Properties {
	const fn new(odr: u32, noise: f32, alpha: f32) -> Self {
		Self {
			time: 0, odr, prng: 0x112DECAF, noise, alpha, idx: 0
		}
	}
}

#[derive(Clone, Copy, Default)]
struct Snapshot<T> {
	time: u32,
	data: T
}

pub struct Sensor<const N: usize, T: Copy + Default, S: SensorKind<T> + Default> {
	prop: Properties,
	kind: S,
	hist: [Snapshot<T>; N],
	count: usize
}

impl<const N: usize, T: Copy + Default, S: SensorKind<T> + Default>
Sensor<N, T, S> {
	pub fn new(odr: u32, noise: f32, alpha: f32, value: T) -> Self {
		assert!(N > 0);
		let mut dev = Sensor {
			prop: Properties::new(odr, noise, alpha),
			kind: S::default(),
			hist: [Snapshot::<T>::default(); N],
			count: 1
		};
		dev.hist[0] = Snapshot { time: 0, data: value };
		dev.kind.init(value);
		dev
	}

	pub fn then(&mut self, ts: u32, value: T) -> &mut Self {
		if self.count < N {
			let prev = self.hist[self.count - 1].time;
			self.hist[self.count].time = prev + ts;
			self.hist[self.count].data = value;
			self.count += 1;
		}
		self
	}

	fn target(&mut self, elapsed: u32) -> T {
		while self.prop.idx < self.count - 1 && elapsed > self.hist[self.prop.idx].time {
			self.prop.idx += 1;
		}

		self.hist[self.prop.idx].data
	}

	pub fn get(&mut self, time: u32) -> Result<T, T> {
		if time.wrapping_sub(self.prop.time) < self.prop.odr {
			return Err(self.kind.reading())
		}

		let dt_secs = self.prop.odr as f32 / 1000.0;
		let target = self.target(self.prop.time);

		while time.wrapping_sub(self.prop.time) >= self.prop.odr {
			self.prop.time = self.prop.time.wrapping_add(self.prop.odr);
			self.kind.step(target, &mut self.prop, dt_secs);
		}

		Ok(self.kind.reading())
	}
}

// Accelerometer object state

impl<const N: usize> Sensor<N, XYZ, Accl> {
	pub fn system_velocity(&mut self, time: u32) -> XYZ {
		let _ = self.get(time);
		return self.kind.current_velocity();
	}

	pub fn system_position(&mut self, time: u32) -> XYZ {
		let _ = self.get(time);
		return self.kind.current_position();
	}
}