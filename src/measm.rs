// src/measm.rs

// Utilities

fn lcg_normalized(state: &mut u32) -> f32 {
	*state = state.wrapping_mul(1664525).wrapping_add(1013904223);
	(*state as f32 / u32::MAX as f32) * 2.0 - 1.0
}

// Traits

pub(crate) trait Noisy {
	fn iir(self, target: Self, alpha: f32) -> Self;
	fn noise(self, noise: f32, rng: &mut u32) -> Self;
}

pub(crate) trait Vector {
	fn add(self, other: Self) -> Self;
	fn sub(self, other: Self) -> Self;
	fn _add_scalar(self, scalar: f32) -> Self;
	fn _sub_scalar(self, scalar: f32) -> Self;
	fn scale(self, scalar: f32) -> Self;
}

// Types

pub(crate) type XYZ = (f32, f32, f32);

impl Noisy for XYZ {
	fn iir(self, target: Self, alpha: f32) -> Self {
		(self.0 + alpha * (target.0 - self.0),
		 self.1 + alpha * (target.1 - self.1),
		 self.2 + alpha * (target.2 - self.2))
	}

	fn noise(mut self, noise: f32, rng: &mut u32) -> Self {
		self.0 += lcg_normalized(rng) * noise;
		self.1 += lcg_normalized(rng) * noise;
		self.2 += lcg_normalized(rng) * noise;
		self
	}
}

impl Vector for XYZ {
	fn add(self, other: Self) -> Self {
		(self.0 + other.0, self.1 + other.1, self.2 + other.2)
	}

	fn sub(self, other: Self) -> Self {
		(self.0 - other.0, self.1 - other.1, self.2 - other.2)
	}

	fn _add_scalar(self, scalar: f32) -> Self {
		(self.0 + scalar, self.1 + scalar, self.2 + scalar)
	}

	fn _sub_scalar(self, scalar: f32) -> Self {
		(self.0 - scalar, self.1 - scalar, self.2 - scalar)
	}

	fn scale(self, scalar: f32) -> Self {
		(self.0 * scalar, self.1 * scalar, self.2 * scalar)
	}
}