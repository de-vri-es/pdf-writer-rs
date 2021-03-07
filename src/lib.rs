pub const A4: Size2<Mm> = Size2::new(210.0, 297.0);

pub use euclid;

pub struct Mm {
	_private: (),
}

pub struct Pt {
	_private: (),
}

struct PangoUnit {
	_private: (),
}

pub const MM_PER_PT: euclid::Scale<f64, Pt, Mm> = euclid::Scale::new(25.4 / 72.0);
pub const PT_PER_MM: euclid::Scale<f64, Mm, Pt> = euclid::Scale::new(72.0 / 25.4);
const PANGO_PER_PT: euclid::Scale<f64, Pt, PangoUnit> = euclid::Scale::new(1e3);

pub type Box2<Unit> = euclid::Box2D<f64, Unit>;
pub type Point2<Unit> = euclid::Point2D<f64, Unit>;
pub type Size2<Unit> = euclid::Size2D<f64, Unit>;
pub type Vector2<Unit> = euclid::Vector2D<f64, Unit>;
pub type Length<Unit> = euclid::Length<f64, Unit>;

/// Create a value in millimeters.
pub fn mm(value: f64) -> Length<Mm> {
	Length::new(value)
}

/// Create a points in points (1/72 inch).
pub fn pt(value: f64) -> Length<Pt> {
	Length::new(value)
}

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

#[derive(Debug, Clone)]
pub struct TextStyle<'a> {
	pub font: FontSpec<'a>,
	pub align: TextAlign,
	pub justify: bool,
}

#[derive(Debug, Clone)]
pub struct FontSpec<'a> {
	pub family: &'a str,
	pub size: Length<Pt>,
	pub weight: FontWeight,
	pub style: FontStyle,
}

impl<'a> FontSpec<'a> {
	pub fn new(family: &'a str, size: Length<Pt>, weight: FontWeight, style: FontStyle) -> Self {
		Self {
			family,
			size,
			weight,
			style,
		}
	}

	pub fn plain(family: &'a str, size: Length<Pt>) -> Self {
		Self::new(
			family,
			size,
			FontWeight::Normal,
			FontStyle::Normal,
		)
	}

	pub fn bold(family: &'a str, size: Length<Pt>) -> Self {
		Self::new(
			family,
			size,
			FontWeight::Bold,
			FontStyle::Normal,
		)
	}
}

#[derive(Debug, Copy, Clone)]
pub enum FontWeight {
	Thin,
	UltraLight,
	Light,
	SemiLight,
	Book,
	Normal,
	Medium,
	SemiBold,
	Bold,
	UltraBold,
	Heavy,
	UltraHeavy,
}

impl std::default::Default for FontWeight {
	fn default() -> Self {
		Self::Normal
	}
}

#[derive(Debug, Copy, Clone)]
pub enum FontStyle {
	Normal,
	Oblique,
	Italic,
}

impl std::default::Default for FontStyle {
	fn default() -> Self {
		Self::Normal
	}
}

#[derive(Debug, Copy, Clone)]
pub enum TextAlign {
	Left,
	Center,
	Right,
}

impl std::default::Default for TextAlign {
	fn default() -> Self {
		Self::Left
	}
}

#[derive(Debug, Clone)]
pub struct TextBoxPosition {
	pub point: Point2<Mm>,
	pub x_anchor: HorizontalAnchor,
	pub y_anchor: VerticalAnchor,
}

impl TextBoxPosition {
	pub fn new(point: Point2<Mm>, x_anchor: HorizontalAnchor, y_anchor: VerticalAnchor) -> Self {
		Self { point, x_anchor, y_anchor }
	}

	pub fn at(point: Point2<Mm>) -> Self {
		Self::new(point, HorizontalAnchor::Left, VerticalAnchor::Top)
	}
}

#[derive(Debug, Copy, Clone)]
pub enum VerticalAnchor {
	Top,
	Baseline,
	Middle,
	Bottom,
}

#[derive(Debug, Copy, Clone)]
pub enum HorizontalAnchor {
	Left,
	Middle,
	Right,
}

#[derive(Debug, Clone)]
pub struct TextExtent {
	pub absolute: Box2<Mm>,
	pub logical: Box2<Mm>,
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
		let position = TextBoxPosition {
			point: Point2::new(self.margins[3].get(), self.cursor_y.get()),
			x_anchor: HorizontalAnchor::Left,
			y_anchor: VerticalAnchor::Top,
		};
		let extents = self.draw_text_box(text, style, position, Some(self.text_width()))?;

		self.cursor_y += Length::new(extents.logical.height());
		Ok(())
	}

	pub fn draw_text_box(
		&self,
		text: &str,
		style: &TextStyle,
		position: TextBoxPosition,
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
		let baseline = Length::<Pt>::new(f64::from(layout.get_baseline()) / 1e3) * MM_PER_PT;

		// Compute position offset for rendering the text layout and apply it to the text extents.
		let position_offset = position.point.to_vector() + position.align_offset(logical_extent.size(), baseline);
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

fn box_from_pango(rect: pango::Rectangle) -> Box2<Pt> {
	let position = Point2::new(
		f64::from(rect.x) / 1e3,
		f64::from(rect.y) / 1e3,
	);
	let size = Size2::new(
		f64::from(rect.width) / 1e3,
		f64::from(rect.height) / 1e3,
	);
	Box2::new(position, position + size)
}


impl TextStyle<'_> {
	fn apply_to_layout(&self, layout: &pango::Layout) {
		let font = self.font.to_pango();
		layout.set_font_description(Some(&font));
		layout.set_alignment(self.align.to_pango());
		layout.set_justify(self.justify);
	}
}

impl FontSpec<'_> {
	fn to_pango(&self) -> pango::FontDescription {
		let mut font = pango::FontDescription::new();
		font.set_family(self.family);
		font.set_weight(self.weight.to_pango());
		font.set_style(self.style.to_pango());
		font.set_absolute_size((self.size * PANGO_PER_PT).get());
		font
	}
}

impl FontWeight {
	fn to_pango(self) -> pango::Weight {
		match self {
			Self::Thin => pango::Weight::Thin,
			Self::UltraLight => pango::Weight::Ultralight,
			Self::Light => pango::Weight::Light,
			Self::SemiLight => pango::Weight::Semilight,
			Self::Book => pango::Weight::Book,
			Self::Normal => pango::Weight::Normal,
			Self::Medium => pango::Weight::Medium,
			Self::SemiBold => pango::Weight::Semibold,
			Self::Bold => pango::Weight::Bold,
			Self::UltraBold => pango::Weight::Ultrabold,
			Self::Heavy => pango::Weight::Heavy,
			Self::UltraHeavy => pango::Weight::Ultraheavy,
		}
	}
}

impl FontStyle {
	fn to_pango(self) -> pango::Style {
		match self {
			Self::Normal => pango::Style::Normal,
			Self::Oblique => pango::Style::Oblique,
			Self::Italic => pango::Style::Italic,
		}
	}
}

impl TextAlign {
	fn to_pango(self) -> pango::Alignment {
		match self {
			Self::Left => pango::Alignment::Left,
			Self::Center => pango::Alignment::Center,
			Self::Right => pango::Alignment::Right,
		}
	}
}

impl TextBoxPosition {
	fn align_offset<Unit>(&self, size: Size2<Unit>, baseline: Length<Unit>) -> Vector2<Unit> {
		let x = match self.x_anchor {
			HorizontalAnchor::Left => 0.0,
			HorizontalAnchor::Middle => size.width * -0.5,
			HorizontalAnchor::Right => size.width * -1.0,
		};

		let y = match self.y_anchor {
			VerticalAnchor::Top => 0.0,
			VerticalAnchor::Baseline => -baseline.get(),
			VerticalAnchor::Middle => size.height * -0.5,
			VerticalAnchor::Bottom => size.height * -1.0,
		};

		Vector2::new(x, y)
	}
}
