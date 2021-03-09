pub const A4: Size2<Mm> = Size2::new(210.0, 297.0);

pub use euclid;

mod font_spec;
pub use font_spec::*;

mod text_style;
pub use text_style::*;

mod units;
pub use units::*;

pub struct PdfWriter {
	cairo: cairo::Context,
	pango: pango::Context,
	size: Size2<Mm>,
}

pub struct Page<'a> {
	pdf: &'a mut PdfWriter,
	margins: [Length<Mm>; 4],
	cursor_y: Length<Mm>,
}

impl PdfWriter {
	pub fn new<W: std::io::Write + 'static>(stream: W, size: Size2<Mm>) -> Result<Self, String> {
		let surface = cairo::PdfSurface::for_stream(
			size.width * PT_PER_MM.get(),
			size.height * PT_PER_MM.get(),
			stream
		)
			.map_err(|e| format!("failed to create PDF surface: {}", e))?;
		let cairo = cairo::Context::new(&surface);
		let pango = pango::Context::new();
		let font_map = pangocairo::FontMap::get_default()
			.ok_or_else(|| "failed to get default font map")?;
		pango.set_font_map(&font_map);

		Ok(Self {
			cairo,
			pango,
			size,
		})
	}

	pub fn page(&mut self, margins: [Length<Mm>; 4]) -> Page {
		self.cairo.save();
		Page {
			pdf: self,
			margins,
			cursor_y: margins[0],
		}
	}

	fn load_font(&self, font: &FontSpec) -> Result<(), ()> {
		self.pango.load_font(&font.to_pango()).ok_or(())?;
		Ok(())
	}
}

impl<'a> Page<'a> {
	pub fn text_width(&self) -> Length<Mm> {
		Length::<Mm>::new(self.pdf.size.width) - self.margins[1] - self.margins[3]
	}

	pub fn write_text(&mut self, text: &str, style: &TextStyle) -> Result<(), String> {
		let position = BoxPosition::at_xy(self.margins[3], self.cursor_y);
		let extents = self.draw_text_box(text, style, position, Some(self.text_width()))?;

		self.cursor_y += Length::new(extents.logical.height());
		Ok(())
	}

	pub fn draw_text_box(
		&self,
		text: &str,
		style: &TextStyle,
		position: BoxPosition,
		width: Option<Length<Mm>>,
	) -> Result<TextExtent, String> {
		self.pdf.load_font(&style.font)
			.map_err(|()| format!("failed to load font"))?;
		let layout = pangocairo::create_layout(&self.pdf.cairo)
			.ok_or("failed to create pango layout")?;
		style.apply_to_layout(&layout);

		if let Some(width) = width {
			layout.set_width(((width * PT_PER_MM).get() * 1e3).round() as i32);
		}

		layout.set_text(text);

		let (absolute_extent, logical_extent) = layout.get_extents();
		let absolute_extent = box_from_pango(absolute_extent) * MM_PER_PT;
		let logical_extent = box_from_pango(logical_extent) * MM_PER_PT;
		let baseline = Length::<PangoUnit>::new(f64::from(layout.get_baseline())) * PT_PER_PANGO * MM_PER_PT;

		// Compute position offset for rendering the text layout and apply it to the text extents.
		let position_offset = position.point.to_vector() + position.alignment_offset(logical_extent.size(), baseline);
		let logical_extent = logical_extent.translate(position_offset);
		let absolute_extent = absolute_extent.translate(position_offset);

		self.pdf.cairo.move_to(logical_extent.min.x * PT_PER_MM.get(), logical_extent.min.y * PT_PER_MM.get());
		pangocairo::show_layout(&self.pdf.cairo, &layout);

		Ok(TextExtent {
			logical: logical_extent,
			absolute: absolute_extent,
		})
	}

	/// Emit the page.
	pub fn emit(self) {
		drop(self)
	}

	/// Emit the page without clearing the page canvas.
	///
	/// You can continue drawing on the current page and emit it again.
	pub fn copy(&self) {
		self.pdf.cairo.copy_page();
	}

	/// Clear the page contents.
	pub fn clear(&mut self) {
		self.pdf.cairo.save();
		self.pdf.cairo.set_operator(cairo::Operator::Clear);
		self.pdf.cairo.rectangle(0.0, 0.0, self.pdf.size.width * MM_PER_PT.get(), self.pdf.size.height * MM_PER_PT.get());
		self.pdf.cairo.paint_with_alpha(1.0);
		self.pdf.cairo.restore();
	}

	/// Discard the page without adding it to the document.
	pub fn discard(mut self) {
		self.clear();
		std::mem::forget(self);
	}
}

impl<'a> Drop for Page<'a> {
	fn drop(&mut self) {
		self.pdf.cairo.show_page();
	}
}
