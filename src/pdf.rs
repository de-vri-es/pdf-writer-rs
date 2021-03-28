use crate::{A4, Context, Drawable, IntoLength, Length, Margins, Rectangle, Surface, Vector2};

pub struct PdfWriter {
	pdf: cairo::PdfSurface,
	surface: Surface,
}

pub struct Page {
	size: Vector2,
	margins: Margins,
	surface: Surface,
}

impl PdfWriter {
	pub fn new<W: std::io::Write + 'static>(stream: W) -> Result<Self, String> {
		let pdf = cairo::PdfSurface::for_stream(A4.x.as_pt(), A4.y.as_pt(), stream)
			.map_err(|e| format!("failed to create PDF surface: {}", e))?;
		let surface = Surface::new(&pdf, A4);
		Ok(Self {
			pdf,
			surface,
		})
	}

	pub fn add(&self, page: &Page) -> Result<(), String> {
		self.pdf.set_size(page.size.x.as_pt(), page.size.y.as_pt())
			.map_err(|e| format!("failed to set page size: {}", e))?;
		copy_surface(&self.surface, &page.surface);
		self.surface.cairo.show_page();
		Ok(())
	}
}

impl Page {
	pub fn new(context: &Context) -> Result<Self, String> {
		let surface = cairo::Surface::create_similar(
			&context.fake_pdf,
			cairo::Content::Alpha,
			A4.x.as_device_units(),
			A4.y.as_device_units()
		);
		let surface = surface.map_err(|e| format!("failed to create page surface: {}", e))?;

		Ok(Self {
			size: A4,
			margins: Margins::vh(30.mm(), 20.mm()),
			surface: Surface::new(&surface, A4),
		})
	}

	pub fn surface(&self) -> &Surface {
		&self.surface
	}

	pub fn set_size(&mut self, size: Vector2) -> Result<&mut Self, String> {
		let surface = cairo::Surface::create_similar(
			&self.surface.cairo.get_target(),
			cairo::Content::Alpha,
			size.x.as_device_units(),
			size.y.as_device_units()
		);
		let surface = surface.map_err(|e| format!("failed to create page surface: {}", e))?;
		let surface = Surface::new(&surface, size);
		copy_surface(&surface, &self.surface);

		self.surface = surface;
		Ok(self)
	}

	pub fn set_size_a4(&mut self) -> Result<&mut Self, String> {
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

	pub fn draw<D: Drawable>(&self, drawable: D, position: Vector2) {
		drawable.draw(&self.surface, position);
	}

	/// Clear the page contents.
	pub fn clear(&self) {
		self.surface.cairo.save();
		self.surface.cairo.set_operator(cairo::Operator::Clear);
		self.surface.cairo.rectangle(0.0, 0.0, self.size.x.as_pt(), self.size.y.as_pt());
		self.surface.cairo.paint_with_alpha(1.0);
		self.surface.cairo.restore();
	}
}

fn copy_surface(target: &Surface, source: &Surface) {
	target.cairo.save();
	target.cairo.set_source_surface(&source.cairo.get_target(), 0.0, 0.0);
	target.cairo.rectangle(0.0, 0.0, source.size.x.as_pt(), source.size.y.as_pt());
	target.cairo.fill();
	target.cairo.restore();
}
