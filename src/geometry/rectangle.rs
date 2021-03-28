use super::{Length, Margins, Vector2};

/// A 2D rectangle with known units.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rectangle {
	/// The minimum corner.
	min: Vector2,

	/// The maximum corner.
	max: Vector2,
}

impl Rectangle {
	/// Create a rectangle from the minimum and maximum corner.
	#[inline]
	pub const fn from_min_max(min: Vector2, max: Vector2) -> Self {
		Self { min, max }
	}

	/// Create a rectangle from the position and size.
	#[inline]
	pub fn from_pos_size(position: Vector2, size: Vector2) -> Self {
		Self::from_min_max(position, position + size)
	}

	/// Create a rectangle from the position and size.
	#[inline]
	pub fn from_xywh(x: Length, y: Length, width: Length, height: Length) -> Self {
		Self::from_pos_size(Vector2::new(x, y), Vector2::new(width, height))
	}

	/// Get the minimum corner of the rectangle.
	#[inline]
	pub const fn min(&self) -> Vector2 {
		self.min
	}

	/// Get the maximum corner of the rectangle.
	#[inline]
	pub const fn max(&self) -> Vector2 {
		self.max
	}

	/// Get the size of the rectangle.
	#[inline]
	pub fn size(&self) -> Vector2 {
		self.max - self.min
	}

	/// Shrink the rectangle by the given margins.
	pub fn shrink(&self, margins: &Margins) -> Self {
		let top_left = Vector2::new(margins.left, margins.top);
		let bottom_right = Vector2::new(margins.right, margins.bottom);
		Self::from_min_max(self.min() + top_left, self.max() - bottom_right)
	}

	/// Grow the rectangle by the given margins.
	pub fn grow(&self, margins: &Margins) -> Self {
		let top_left = Vector2::new(margins.left, margins.top);
		let bottom_right = Vector2::new(margins.right, margins.bottom);
		Self::from_min_max(self.min() - top_left, self.max() + bottom_right)
	}
}
