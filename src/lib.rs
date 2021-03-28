mod geometry;
pub use geometry::*;

mod drawables;
pub use drawables::*;

mod pdf;
pub use pdf::*;

pub const A4: Vector2 = Vector2::new(Length::from_mm(210.0), Length::from_mm(297.0));

pub trait Drawable {
	fn draw(&self, surface: &Surface, position: Vector2);

	fn set_max_width(&mut self, width: Option<Length>);

	fn max_width(&self) -> Option<Length>;

	fn compute_size(&self) -> Vector2;

	fn compute_width(&self) -> Length {
		self.compute_size().x
	}

	fn compute_height(&self) -> Length {
		self.compute_size().y
	}

	fn compute_natural_width(&self) -> Length;
}

pub struct Context {
	pango: pango::Context,
}

pub struct Surface {
	cairo: cairo::Context,
}

impl Context {
	pub fn text(&self) -> Text {
		Text::new(self)
	}
}
