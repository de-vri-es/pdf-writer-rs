use crate::{Drawable, DrawableMut, Length, Surface, Vector2};

#[derive(Debug, Clone)]
pub struct Offset<T> {
	inner: T,
	offset: Vector2,
	anchor_x: AnchorX,
	anchor_y: AnchorY,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AnchorX {
	Left,
	Center,
	Right,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AnchorY {
	Top,
	Baseline,
	Center,
	Bottom,
}

impl<T> Offset<T> {
	/// Add a position offset to a [`Drawable`].
	///
	/// When the resulting [`Offset`] is drawn,
	/// it adds the configured offset to the position.
	///
	/// Additionally, you can change the anchor point of the inner drawable.
	/// This can be used to align items differently than on their top-left corner.
	pub fn new(inner: T, offset: Vector2) -> Self {
		Self {
			inner,
			offset,
			anchor_x: AnchorX::Left,
			anchor_y: AnchorY::Top,
		}
	}

	/// Get a reference to the wrapped drawable.
	pub fn inner(&self) -> &T {
		&self.inner
	}

	/// Get a mutable reference to the wrapped drawable.
	pub fn inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	/// Move the wrapped drawable out of the this [`Offset`].
	pub fn into_inner(self) -> T {
		self.inner
	}

	/// Get the position offset.
	pub fn offset(&self) -> &Vector2 {
		&self.offset
	}

	/// Get a mutable reference to the position offset.
	pub fn offset_mut(&mut self) -> &mut Vector2 {
		&mut self.offset
	}

	/// Change the configured position offset.
	pub fn set_offset(mut self, offset: Vector2) -> Self {
		self.offset = offset;
		self
	}

	/// Add a vector to the current position offset.
	pub fn add_offset(mut self, offset: Vector2) -> Self {
		self.offset += offset;
		self
	}

	/// Subtract a vector from the current position offset.
	pub fn sub_offset(mut self, offset: Vector2) -> Self {
		self.offset -= offset;
		self
	}

	/// Change the X anchor of the wrapped drawable.
	pub fn set_anchor_x(mut self, anchor_x: AnchorX) -> Self {
		self.anchor_x = anchor_x;
		self
	}

	/// Change the Y anchor of the wrapped drawable.
	pub fn set_anchor_y(mut self, anchor_y: AnchorY) -> Self {
		self.anchor_y = anchor_y;
		self
	}

	/// Anchor the wrapped drawable on the left.
	pub fn anchor_left(self) -> Self {
		self.set_anchor_x(AnchorX::Left)
	}

	/// Anchor the wrapped drawable on the center in X direction.
	pub fn anchor_center_x(self) -> Self {
		self.set_anchor_x(AnchorX::Center)
	}

	/// Anchor the wrapped drawable on the right.
	pub fn anchor_right(self) -> Self {
		self.set_anchor_x(AnchorX::Right)
	}

	/// Anchor the wrapped drawable on the top.
	pub fn anchor_top(self) -> Self {
		self.set_anchor_y(AnchorY::Top)
	}

	/// Anchor the wrapped drawable on the baseline.
	pub fn anchor_baseline(self) -> Self {
		self.set_anchor_y(AnchorY::Baseline)
	}

	/// Anchor the wrapped drawable on the center in Y direction.
	pub fn anchor_center_y(self) -> Self {
		self.set_anchor_y(AnchorY::Center)
	}

	/// Anchor the wrapped drawable on the bottom.
	pub fn anchor_bottom(self) -> Self {
		self.set_anchor_y(AnchorY::Bottom)
	}
}

impl<T: Drawable> Drawable for Offset<T> {
	fn draw(&self, surface: &Surface, position: Vector2) {
		if self.anchor_x == AnchorX::Left && self.anchor_y == AnchorY::Top {
			self.inner.draw(surface, position + self.offset);
		} else {
			let size = self.compute_size();
			let align_x = match self.anchor_x {
				AnchorX::Left => Length::zero(),
				AnchorX::Center => 0.5 * size.x,
				AnchorX::Right => 1.0 * size.x,
			};
			let align_y = match self.anchor_y {
				AnchorY::Top => Length::zero(),
				AnchorY::Baseline => self.compute_baseline().unwrap_or_else(Length::zero),
				AnchorY::Center => 0.5 * size.y,
				AnchorY::Bottom => 1.0 * size.y,
			};
			let align = Vector2::new(align_x, align_y);
			self.inner.draw(surface, position + self.offset - align);
		}
	}

	fn min_width(&self) -> Length {
		self.inner.min_width()
	}

	fn max_width(&self) -> Option<Length> {
		self.inner.max_width()
	}

	fn compute_size(&self) -> Vector2 {
		self.inner.compute_size()
	}

	fn compute_width(&self) -> Length {
		self.inner.compute_width()
	}

	fn compute_height(&self) -> Length {
		self.inner.compute_height()
	}

	fn compute_baseline(&self) -> Option<Length> {
		self.inner.compute_baseline()
	}

	fn compute_natural_width(&self) -> Length {
		self.inner.compute_natural_width()
	}
}

impl<T: DrawableMut> DrawableMut for Offset<T> {
	fn set_max_width(&mut self, width: Option<Length>) {
		self.inner.set_max_width(width)
	}
}

impl<T> AsRef<T> for Offset<T> {
	fn as_ref(&self) -> &T {
		self.inner()
	}
}

impl<T> AsMut<T> for Offset<T> {
	fn as_mut(&mut self) -> &mut T {
		self.inner_mut()
	}
}

impl<T: DrawableMut + Sized + 'static> From<Offset<T>> for Box<dyn DrawableMut> {
	fn from(other: Offset<T>) -> Self {
		Box::new(other)
	}
}
