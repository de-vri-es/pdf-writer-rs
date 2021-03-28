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
	fake_pdf: cairo::PdfSurface,
}

pub struct Surface {
	cairo: cairo::Context,
	size: Vector2,
}

impl Context {
	pub fn new() -> Result<Self, String> {
		let pango = pango::Context::new();

		let font_map = pangocairo::FontMap::get_default()
			.ok_or("failed to get default font map")?;
		pango.set_font_map(&font_map);
		pango.load_font(&FontSpec::default().to_pango()).unwrap();

		let fake_pdf = cairo::PdfSurface::for_stream(100.0, 100.0, Vec::new())
			.map_err(|e| format!("failed to create PDF surface: {}", e))?;

		Ok(Self { pango, fake_pdf })
	}

	pub fn pdf<W: std::io::Write + 'static>(&self, stream: W) -> Result<PdfWriter, String> {
		PdfWriter::new(stream)
	}

	pub fn page(&self) -> Result<Page, String> {
		Page::new(self)
	}

	pub fn text_box(&self) -> TextBox {
		TextBox::new(self)
	}
}

impl Surface {
	fn new(surface: &cairo::Surface, size: Vector2) -> Self {
		Self {
			cairo: cairo::Context::new(surface),
			size,
		}
	}
}
