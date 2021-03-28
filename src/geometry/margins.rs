use super::Length;

/// Top, bottom, left and right margnins.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Margins {
	pub top: Length,
	pub bottom: Length,
	pub left: Length,
	pub right: Length,
}

impl Margins {
	/// Create a new `Margins` object from the individual top, bottom left and right margin.
	pub const fn tblr(top: Length, bottom: Length, left: Length, right: Length) -> Self {
		Self { top, bottom, left, right }
	}

	/// Create a new `Margins` object from vertical and horizontal margin.
	///
	/// The vertical margin is duplicated for the top and bottom margin,
	/// and the horizontal margin is duplicated for the left and right margin.
	pub const fn vh(vertical: Length, horizontal: Length) -> Self {
		Self::tblr(vertical, vertical, horizontal, horizontal)
	}

	/// Create a new `Margins` object using the same margin for all sides.
	///
	/// The margin is duplicated for the top, bottom, left and right margin.
	pub const fn uniform(all: Length) -> Self {
		Self::vh(all, all)
	}

	/// Get the total vertical margin.
	///
	/// This is the sum of the top and bottom margin.
	pub fn total_vertical(&self) -> Length {
		self.top + self.bottom
	}

	/// Get the total horizontal margin.
	///
	/// This is the sum of the left and right margin.
	pub fn total_horizontal(&self) -> Length {
		self.left + self.right
	}
}
