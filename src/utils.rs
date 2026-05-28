// src/utils.rs

pub fn lcg_normalized(state: &mut u32) -> f32 {
	*state = state.wrapping_mul(1664525).wrapping_add(1013904223);
	(*state as f32 / u32::MAX as f32) * 2.0 - 1.0
}