mod geometry;
pub use geometry::*;

pub trait Drawable {
	fn draw(&self, surface: &mut Surface, position: Vector2);

	fn set_max_width(&mut self, width: Option<Length>);

	fn get_max_width(&self) -> Option<Length>;

	fn size(&self) -> Vector2;

	fn width(&self) -> Length {
		self.size().x
	}

	fn height(&self) -> Length {
		self.size().y
	}
}

pub struct Context {
	pango: pango::Context,
}

pub struct Surface {
	cairo: cairo::Context,
}
