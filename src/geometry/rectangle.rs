use super::{Length, Margins, Vector2};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rectangle {
	min: Vector2,
	max: Vector2,
}

impl Rectangle {
	#[inline]
	pub const fn from_min_max(min: Vector2, max: Vector2) -> Self {
		Self { min, max }
	}

	#[inline]
	pub fn from_pos_size(position: Vector2, size: Vector2) -> Self {
		Self::from_min_max(position, position + size)
	}

	#[inline]
	pub fn from_xywh(x: Length, y: Length, width: Length, height: Length) -> Self {
		Self::from_pos_size(Vector2::new(x, y), Vector2::new(width, height))
	}

	#[inline]
	pub const fn min(&self) -> Vector2 {
		self.min
	}

	#[inline]
	pub const fn max(&self) -> Vector2 {
		self.max
	}

	#[inline]
	pub fn size(&self) -> Vector2 {
		self.max - self.min
	}

	pub fn shrink(&self, margins: &Margins) -> Self {
		let top_left = Vector2::new(margins.left, margins.top);
		let bottom_right = Vector2::new(margins.right, margins.bottom);
		Self::from_min_max(self.min() + top_left, self.max() - bottom_right)
	}

	pub fn grow(&self, margins: &Margins) -> Self {
		let top_left = Vector2::new(margins.left, margins.top);
		let bottom_right = Vector2::new(margins.right, margins.bottom);
		Self::from_min_max(self.min() - top_left, self.max() + bottom_right)
	}
}
