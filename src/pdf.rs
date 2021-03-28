use crate::{A4, Drawable, IntoLength, Length, Margins, Rectangle, Surface, Vector2};

pub struct PdfWriter {
	pdf: cairo::PdfSurface,
	surface: Surface,
}

pub struct Page {
	size: Vector2,
	margins: Margins,
	drawables: Vec<Box<dyn Drawable>>,
}

impl PdfWriter {
	pub fn new<W: std::io::Write + 'static>(stream: W) -> Result<Self, String> {
		let pdf = cairo::PdfSurface::for_stream(100.0, 100.0, stream)
			.map_err(|e| format!("failed to create PDF surface: {}", e))?;
		let surface = Surface {
			cairo: cairo::Context::new(&pdf),
		};
		Ok(Self {
			pdf,
			surface,
		})
	}

	pub fn add(&self, page: &Page) -> Result<(), String> {
		self.pdf.set_size(page.size.x.as_pt(), page.size.y.as_pt())
			.map_err(|e| format!("failed to set page size: {}", e))?;
		for drawable in &page.drawables {
			drawable.draw(&self.surface, Vector2::zero());
		}
		self.surface.cairo.show_page();
		Ok(())
	}
}

impl Page {
	pub fn new() -> Self {
		Self {
			size: A4,
			margins: Margins::vh(30.mm(), 20.mm()),
			drawables: Vec::new(),
		}
	}

	pub fn set_size(&mut self, size: Vector2) -> &mut Self {
		self.size = size;
		self
	}

	pub fn set_size_a4(&mut self) -> &mut Self {
		self.set_size(A4)
	}

	pub fn set_margins(&mut self, margins: Margins) -> &mut Self {
		self.margins = margins;
		self
	}

	pub fn set_top_margin(&mut self, value: Length) -> &mut Self {
		self.margins.top = value;
		self
	}

	pub fn set_bottom_margin(&mut self, value: Length) -> &mut Self {
		self.margins.bottom = value;
		self
	}

	pub fn set_left_margin(&mut self, value: Length) -> &mut Self {
		self.margins.left = value;
		self
	}

	pub fn set_right_margin(&mut self, value: Length) -> &mut Self {
		self.margins.right = value;
		self
	}

	pub fn set_vertical_margins(&mut self, value: Length) -> &mut Self {
		self.margins.top = value;
		self.margins.bottom = value;
		self
	}

	pub fn set_horizontal_margins(&mut self, value: Length) -> &mut Self {
		self.margins.left = value;
		self.margins.right = value;
		self
	}

	pub fn text_area(&self) -> Rectangle {
		Rectangle::from_min_max(Vector2::zero(), self.size)
			.shrink(&self.margins)
	}

	pub fn text_width(&self) -> Length {
		self.size.x - self.margins.total_horizontal()
	}

	pub fn text_height(&self) -> Length {
		self.size.y - self.margins.total_vertical()
	}

	pub fn add<D: Drawable + 'static>(&mut self, drawable: D) {
		self.drawables.push(Box::new(drawable))
	}

	/// Clear the page contents.
	pub fn clear(&mut self) {
		self.drawables.clear()
	}
}

impl Default for Page {
	fn default() -> Self {
		Self::new()
	}
}
