mod geometry;
pub use geometry::*;

mod drawables;
pub use drawables::*;

mod pdf;
pub use pdf::*;

pub const A4: Vector2 = Vector2::new(Length::from_mm(210.0), Length::from_mm(297.0));

/// Drawable item.
///
/// All drawables are modeled as items that can be limited at a maximum width.
/// Items will automatically grow in height as needed, to compensate for lost width.
pub trait Drawable {
	/// Draw the item to a surface at a given position.
	fn draw(&self, surface: &Surface, position: Vector2);

	/// Get the minimum width of the item.
	fn min_width(&self) -> Length;

	/// Get the maximum width of the item.
	fn max_width(&self) -> Option<Length>;

	/// Compute the size of the item for the current configuration.
	fn compute_size(&self) -> Vector2;

	/// Compute the width of the item for the current configuration.
	///
	/// This is the same as the `x` component returned by [`Drawable::compute_size`],
	/// but it may be more efficient because the height does not need to be computed.
	fn compute_width(&self) -> Length {
		self.compute_size().x
	}

	/// Compute the height of the item for the current configuration.
	///
	/// This is the same as the `x` component returned by [`Drawable::compute_size`],
	/// but it may be more efficient because the width does not need to be computed.
	/// (Although most items will need to compute their width in order to compute the height.)
	fn compute_height(&self) -> Length {
		self.compute_size().y
	}

	/// Compute the distance of the baseline from the top of the drawable.
	///
	/// If the drawable has no baseline (because it has no text), this should return None.
	fn compute_baseline(&self) -> Option<Length>;

	/// Compute the natural width of the item.
	///
	/// This is the computed width of the item if no width limit is applied.
	fn compute_natural_width(&self) -> Length;
}

/// Drawable item (mutable interface).
///
/// All drawables are modeled as items that can be limited at a maximum width.
/// Items will automatically grow in height as needed, to compensate for lost width.
pub trait DrawableMut {
	/// Set the maximum width of the item.
	fn set_max_width(&mut self, width: Option<Length>);
}

/// A context to create PDFs.
pub struct Context {
	pango: pango::Context,
	fake_pdf: cairo::PdfSurface,
}

/// A surface to draw [`Drawable`] items on.
pub struct Surface {
	cairo: cairo::Context,
	size: Vector2,
}

impl Context {
	/// Create a new context.
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

	/// Create a new PDF backed by a [`Write`] stream.
	pub fn pdf<W: std::io::Write + 'static>(&self, stream: W) -> Result<PdfWriter, String> {
		PdfWriter::new(stream)
	}

	/// Create a new page.
	pub fn page(&self) -> Result<Page, String> {
		Page::new(self)
	}

	/// Create a new text box.
	pub fn text_box(&self) -> TextBox {
		TextBox::new(self)
	}
}

impl Surface {
	/// Wrap a cairo surface.
	fn new(surface: &cairo::Surface, size: Vector2) -> Self {
		Self {
			cairo: cairo::Context::new(surface),
			size,
		}
	}
}

impl AsRef<Surface> for Surface {
	fn as_ref(&self) -> &Surface {
		self
	}
}
