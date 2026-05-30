// src/measm.rs

use crate::utils::lcg_normalized;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Triple {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Triple {
	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Self {x, y, z}
	}

	pub(crate) fn iir(prev: Self, target: Self, alpha: f32) -> Self {
		Self {
			x: prev.x + alpha * (target.x - prev.x),
			y: prev.y + alpha * (target.y - prev.y),
			z: prev.z + alpha * (target.z - prev.z)
		}
	}

	pub(crate) fn noise(mut self, noise: f32, rng: &mut u32) -> Self {
		self.x += lcg_normalized(rng) * noise;
		self.y += lcg_normalized(rng) * noise;
		self.z += lcg_normalized(rng) * noise;
		self
	}

	pub fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}

	pub fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}

	pub fn add_scalar(self, scalar: f32) -> Self {
		Self::new(self.x + scalar, self.y + scalar, self.z + scalar)
	}

	pub fn sub_scalar(self, scalar: f32) -> Self {
		Self::new(self.x - scalar, self.y - scalar, self.z - scalar)
	}

	pub fn scale(self, scalar: f32) -> Self {
		Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
	}
}