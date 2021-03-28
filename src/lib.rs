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
	pub fn new() -> Result<Self, String> {
		let pango = pango::Context::new();
		let font_map = pangocairo::FontMap::get_default()
			.ok_or("failed to get default font map")?;
		pango.set_font_map(&font_map);
		pango.load_font(&FontSpec::default().to_pango()).unwrap();
		Ok(Self { pango })
	}

	pub fn pdf<W: std::io::Write + 'static>(&self, stream: W) -> Result<PdfWriter, String> {
		PdfWriter::new(stream)
	}

	pub fn page(&self) -> Page {
		Page::default()
	}

	pub fn text_box(&self) -> TextBox {
		TextBox::new(self)
	}
}
