use crate::drawables::TextBox;
use crate::{Context, Drawable, DrawableMut, Length, Surface, Vector2};

pub struct ItemList {
	bullet: Box<dyn Drawable>,
	bullet_width: Length,
	bullet_spacing: Length,
	items: Vec<Box<dyn DrawableMut>>,
	max_width: Option<Length>,
	min_text_width: Length,
	natural_text_width: Length,
}

impl ItemList {
	pub fn new(context: &Context, bullet_font: &crate::FontSpec) -> Self {
		let bullet = TextBox::new(context).set_text("•").set_font(bullet_font);
		let bullet_width = bullet.compute_width();
		Self {
			bullet: Box::new(bullet),
			bullet_width,
			bullet_spacing: bullet_width,
			items: Vec::new(),
			max_width: None,
			min_text_width: Length::zero(),
			natural_text_width: Length::zero(),
		}
	}

	#[inline]
	pub fn min_width(&self) -> Length {
		if self.is_empty() {
			Length::zero()
		} else {
			self.min_text_width + self.bullet_width + self.bullet_spacing
		}
	}

	#[inline]
	pub fn max_width(&self) -> Option<Length> {
		self.max_width
	}

	#[inline]
	pub fn items(&self) -> &[Box<dyn DrawableMut>] {
		&self.items
	}

	#[inline]
	pub fn clear(&mut self) {
		self.items.clear();
		self.min_text_width = Length::zero();
		self.natural_text_width = Length::zero();
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}

	pub fn compute_size(&self) -> Vector2 {
		if self.is_empty() {
			return Vector2::zero();
		}
		let mut size = Vector2::zero();
		for item in self.items() {
			let item_size = item.compute_size();
			size.x = size.x.max(item_size.x);
			size.y += item_size.y;
		}

		size.x += self.bullet_width + self.bullet_spacing;
		size
	}

	#[inline]
	pub fn compute_natural_width(&self) -> Length{
		if self.is_empty() {
			Length::zero()
		} else {
			self.natural_text_width + self.bullet_width + self.bullet_spacing
		}
	}

	#[inline]
	pub fn draw_bullet(&self, surface: &Surface, position: Vector2) {
		self.bullet.draw(surface, position)
	}

	#[inline]
	pub fn draw_item(&self, index: usize, surface: &Surface, position: Vector2) {
		self.draw_bullet(surface, position);
		self.items[index].draw(surface, position + Vector2::new(self.bullet_width + self.bullet_spacing, Length::from_mm(0.0)));
	}

	#[inline]
	pub fn set_max_width(mut self, max_width: Option<Length>) -> Self {
		self._set_max_width(max_width);
		self
	}

	fn _set_max_width(&mut self, max_width: Option<Length>) {
		let max_text_width = max_width.map(|x| x - self.bullet_width - self.bullet_spacing);
		for item in &mut self.items {
			item.set_max_width(max_text_width);
		}
		self.max_width = max_width;
	}

	#[inline]
	pub fn add_item<T: Into<Box<dyn DrawableMut>>>(mut self, item: T) -> Self {
		let mut item = item.into();
		self.natural_text_width = self.natural_text_width.max(item.compute_natural_width());
		self.min_text_width = self.min_text_width.max(item.min_width());
		item.set_max_width(self.max_width);
		self.items.push(item);
		self
	}
}

impl Drawable for ItemList {
	fn draw(&self, surface: &Surface, position: Vector2) {
		let mut position = position;
		for index in 0..self.items.len() {
			self.draw_item(index, surface, position);
			position.y += self.items[index].compute_height();
		}
	}

	#[inline]
	fn min_width(&self) -> Length {
		self.min_width()
	}

	#[inline]
	fn max_width(&self) -> Option<Length> {
		self.max_width()
	}

	#[inline]
	fn compute_size(&self) -> Vector2 {
		self.compute_size()
	}

	#[inline]
	fn compute_baseline(&self) -> Option<Length> {
		if self.items.is_empty() {
			None
		} else {
			self.items[0].compute_baseline()
		}
	}

	#[inline]
	fn compute_natural_width(&self) -> Length {
		self.compute_natural_width()
	}
}

impl DrawableMut for ItemList {
	#[inline]
	fn set_max_width(&mut self, width: Option<Length>) {
		self._set_max_width(width);
	}
}

impl From<ItemList> for Box<dyn DrawableMut> {
	fn from(other: ItemList) -> Self {
		Box::new(other)
	}
}
