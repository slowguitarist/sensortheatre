// src/scenario.rs

use crate::measm::Triple;

#[derive(Clone, Copy, Default)]
pub struct Snapshot {
	pub duration: u32,
	pub accl: Triple,
	pub gyro: Triple
}

pub struct Scenario<const N: usize> {
	pub points: [Snapshot; N],
	pub count: usize
}

impl<const N: usize> Scenario<N> {
	pub fn init(accl: Triple, gyro: Triple) -> Self {
		let mut k = Self {
			points: [Snapshot::default(); N],
			count: 1
		};
		k.points[0] = Snapshot { duration: 0, accl, gyro };
		k
	}

	pub fn then(mut self, ms: u32, accl: Triple, gyro: Triple) -> Self {
		if self.count < N {
			self.points[self.count] = Snapshot { duration: ms, accl, gyro };
			self.count += 1;
		}
		self
	}

	pub fn target(&self, elapsed: u32) -> (Triple, Triple) {
		let mut accum = 0;

		for k in 1..self.count {
			accum += self.points[k].duration;
			if elapsed <= accum {
				return (self.points[k].accl, self.points[k].gyro);
			}
		}

		let last_known = self.points[self.count - 1];
		(last_known.accl, last_known.gyro)
	}
}