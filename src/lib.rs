pub const A4: Size2<Mm> = Size2::new(210.0, 297.0);

pub use euclid;

mod font_spec;
pub use font_spec::*;

mod text_style;
pub use text_style::*;

mod units;
pub use units::*;

mod table;
pub use table::*;

pub struct PdfWriter {
	surface: cairo::PdfSurface,
	cairo: cairo::Context,
}

pub struct Page {
	cairo: cairo::Context,
	size: Size2<Mm>,
	margins: Margins<Mm>,
	cursor_y: Length<Mm>,
}

impl PdfWriter {
	pub fn new<W: std::io::Write + 'static>(stream: W) -> Result<Self, String> {
		let surface = cairo::PdfSurface::for_stream(
			100.0,
			100.0,
			stream
		);
		let surface = surface.map_err(|e| format!("failed to create PDF surface: {}", e))?;
		let cairo = cairo::Context::new(&surface);
		let pango = pango::Context::new();
		let font_map = pangocairo::FontMap::get_default()
			.ok_or_else(|| "failed to get default font map")?;
		pango.set_font_map(&font_map);

		Ok(Self {
			surface,
			cairo,
		})
	}

	pub fn text_box(
		&self,
		text: &str,
		style: &TextStyle,
		position: BoxPosition,
		width: Option<Length<Mm>>,
	) -> Result<TextBox, String> {
		TextBox::new(&self.cairo, text, style, position, width)
	}

	pub fn page(&mut self, size: Size2<Mm>, margins: Margins<Mm>) -> Result<Page, String> {
		let device_size = size * PT_PER_MM * PANGO_PER_PT;
		let width = device_size.width.round() as i32;
		let height = device_size.height.round() as i32;

		let buffer = self.cairo
			.get_target()
			.create_similar(cairo::Content::Alpha, width, height)
			.map_err(|e| format!("failed to create buffer surface for page: {}", e))?;
		let cairo = cairo::Context::new(&buffer);
		let cursor_y = margins.top;
		Ok(Page {
			cairo,
			size,
			margins,
			cursor_y,
		})
	}
}

impl Page {
	pub fn text_width(&self) -> Length<Mm> {
		Length::<Mm>::new(self.size.width) - self.margins.left - self.margins.right
	}

	pub fn cursor(&self) -> Point2<Mm> {
		Point2::new(
			self.margins.left.get(),
			self.cursor_y.get(),
		)
	}

	pub fn line_left(&self) -> Point2<Mm> {
		Point2::new(
			self.margins.left.get(),
			self.cursor_y.get(),
		)
	}

	pub fn line_right(&self) -> Point2<Mm> {
		Point2::new(
			self.size.width - self.margins.right.get(),
			self.cursor_y.get(),
		)
	}

	pub fn line_center(&self) -> Point2<Mm> {
		Point2::new(
			(self.margins.left + self.text_width() * 0.5).get(),
			self.cursor_y.get(),
		)
	}

	pub fn write_text(&mut self, text: &str, style: &TextStyle) -> Result<(), String> {
		let position = BoxPosition::at_xy(self.margins.left, self.cursor_y);
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
		let extents = TextBox::new(&self.cairo, text, style, position, width)?.draw(self);
		Ok(extents)
	}

	/// Emit the page.
	pub fn emit(&self, pdf: &PdfWriter) -> Result<(), String> {
		let size_pt = self.size * PT_PER_MM;
		pdf.cairo.save();
		pdf.surface.set_size(size_pt.width, size_pt.height)
			.map_err(|e| format!("failed to set page size: {}", e))?;
		pdf.cairo.set_source_surface(&self.cairo.get_target(), 0.0, 0.0);
		pdf.cairo.rectangle(0.0, 0.0, size_pt.width, size_pt.height);
		pdf.cairo.fill();
		pdf.cairo.restore();
		pdf.cairo.show_page();
		Ok(())
	}

	/// Clear the page contents.
	pub fn clear(&mut self) {
		let size_pt = self.size * PT_PER_MM;
		self.cairo.save();
		self.cairo.set_operator(cairo::Operator::Clear);
		self.cairo.rectangle(0.0, 0.0, size_pt.width, size_pt.height);
		self.cairo.paint_with_alpha(1.0);
		self.cairo.restore();
	}
}

/// A text box that can be rendered to a page.
pub struct TextBox {
	layout: pango::Layout,
	position: BoxPosition,
}

impl TextBox {
	fn new(cairo: &cairo::Context, text: &str, style: &TextStyle, position: BoxPosition, width: Option<Length<Mm>>) -> Result<Self, String> {
		let layout = pangocairo::create_layout(cairo)
			.ok_or("failed to create pango layout")?;
		load_font(&layout, &style.font)?;
		style.apply_to_layout(&layout);

		if let Some(width) = width {
			layout.set_width(((width * PT_PER_MM * PANGO_PER_PT).get()).round() as i32);
		}

		layout.set_text(text);

		Ok(Self {
			layout,
			position,
		})
	}

	/// Compute the logical and absolute extents of the text box with the current parameters.
	pub fn compute_extents(&self) -> TextExtent {
		let (absolute_extent, logical_extent) = self.layout.get_extents();
		let absolute_extent = box_from_pango(absolute_extent) * MM_PER_PT;
		let logical_extent = box_from_pango(logical_extent) * MM_PER_PT;
		let baseline = Length::<PangoUnit>::new(f64::from(self.layout.get_baseline())) * PT_PER_PANGO * MM_PER_PT;

		// Compute position offset for rendering the text layout and apply it to the text extents.
		let position_offset = self.position.point.to_vector() + self.position.alignment_offset(logical_extent.size(), baseline);
		let logical_extent = logical_extent.translate(position_offset);
		let absolute_extent = absolute_extent.translate(position_offset);

		TextExtent {
			logical: logical_extent,
			absolute: absolute_extent,
		}
	}

	/// Compute the logical width of the text.
	///
	/// This is somewhat cheaper than using `compute_extents`,
	/// since it does not need to take into accounts the box position.
	pub fn logical_width(&self) -> Length<Mm> {
		let (_, logical) = self.layout.get_extents();
		Length::<PangoUnit>::new(logical.width.into()) * PT_PER_PANGO * MM_PER_PT
	}

	/// Compute the logical height of the text.
	///
	/// This is somewhat cheaper than using `compute_extents`,
	/// since it does not need to take into accounts the box position.
	pub fn logical_height(&self) -> Length<Mm> {
		let (_, logical) = self.layout.get_extents();
		Length::<PangoUnit>::new(logical.height.into()) * PT_PER_PANGO * MM_PER_PT
	}

	/// Get the baseline of the text.
	pub fn baseline(&self) -> Length<Mm> {
		Length::<PangoUnit>::new(self.layout.get_baseline().into())
			* PT_PER_PANGO
			* MM_PER_PT
	}

	/// Set the style of the text box.
	pub fn set_style(&mut self, style: &TextStyle) -> Result<(), String> {
		load_font(&self.layout, &style.font)?;
		style.apply_to_layout(&self.layout);
		Ok(())
	}

	/// Get the position of the text box.
	pub fn position(&self) -> &BoxPosition {
		&self.position
	}

	/// Set the position of the text box.
	pub fn set_position(&mut self, position: BoxPosition) {
		self.position = position;
	}

	/// Set the width of the text box.
	///
	/// If the width is `None`, no line-wrapping is performend and the text box will grow in width to fit the text.
	pub fn set_width(&mut self, width: Option<Length<Mm>>) {
		if let Some(width) = width {
			self.layout.set_width(((width * PT_PER_MM * PANGO_PER_PT).get()).round() as i32);
		} else {
			self.layout.set_width(0);
		}
	}

	/// Draw the text on a page.
	pub fn draw(&self, page: &Page) -> TextExtent {
		self.draw_offset(page, Vector2::new(0.0, 0.0))
	}

	/// Draw the text on a page.
	pub fn draw_offset(&self, page: &Page, offset: Vector2<Mm>) -> TextExtent {
		let mut extents = self.compute_extents();
		extents.logical = extents.logical.translate(offset);
		extents.absolute = extents.absolute.translate(offset);

		let position = extents.logical.min * PT_PER_MM;
		page.cairo.move_to(position.x, position.y);
		pangocairo::show_layout(&page.cairo, &self.layout);
		extents
	}
}

fn load_font(layout: &pango::Layout, font: &FontSpec) -> Result<(), String> {
	let pango = layout.get_context()
		.ok_or("failed to get pango context for text layout")?;
	pango.load_font(&font.to_pango())
		.ok_or("failed to load font")?;
	Ok(())
}
