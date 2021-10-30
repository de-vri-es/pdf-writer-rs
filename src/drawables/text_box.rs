use crate::{Color, Context, Drawable, DrawableMut, Length, Surface, Vector2, device_units};

/// A drawable text box.
pub struct TextBox {
	layout: pango::Layout,
	color: Color,
}

impl TextBox {
	/// Create a new text box.
	#[inline]
	pub fn new(context: &Context) -> Self {
		let layout = pango::Layout::new(&context.pango);
		TextStyle::default().apply_to_layout(&layout);
		let color = Color::black();
		Self { layout, color }
	}

	/// Set the text.
	#[inline]
	pub fn set_text(self, text: &str) -> Self {
		self.layout.set_text(text);
		self
	}

	/// Set the text color.
	#[inline]
	pub fn set_color(mut self, color: Color) -> Self {
		self.color = color;
		self
	}

	/// Set the complete text style.
	#[inline]
	pub fn set_style(self, style: &TextStyle) -> Self {
		style.apply_to_layout(&self.layout);
		self
	}

	/// Set the complete font specification.
	#[inline]
	pub fn set_font(self, font: &FontSpec) -> Self {
		self.layout.set_font_description(Some(&font.to_pango()));
		self
	}

	/// Set the font family.
	#[inline]
	pub fn set_font_family(self, family: &str) -> Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.font_description().unwrap();
		font.set_family(family);
		self.layout.set_font_description(Some(&font));
		self
	}

	/// Set the font size.
	#[inline]
	pub fn set_font_size(self, size: Length) -> Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.font_description().unwrap();
		font.set_absolute_size(size.as_device_units_f64());
		self.layout.set_font_description(Some(&font));
		self
	}

	/// Set the font weight.
	#[inline]
	pub fn set_font_weight(self, weight: FontWeight) -> Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.font_description().unwrap();
		font.set_weight(weight.to_pango());
		self.layout.set_font_description(Some(&font));
		self
	}

	/// Set the font weight to [`FontWeight::Bold`].
	#[inline]
	pub fn make_bold(self) -> Self {
		self.set_font_weight(FontWeight::Bold)
	}

	/// Set the font weight to [`FontWeight::Thin`].
	#[inline]
	pub fn make_thin(self) -> Self {
		self.set_font_weight(FontWeight::Thin)
	}

	/// Set the font style.
	#[inline]
	pub fn set_font_style(self, style: FontStyle) -> Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.font_description().unwrap();
		font.set_style(style.to_pango());
		self.layout.set_font_description(Some(&font));
		self
	}

	/// Set the font style to [`FontStyle::Italic`].
	#[inline]
	pub fn make_italic(self) -> Self {
		self.set_font_style(FontStyle::Italic)
	}

	/// Set the font style to [`FontStyle::Oblique`].
	#[inline]
	pub fn make_oblique(self) -> Self {
		self.set_font_style(FontStyle::Oblique)
	}

	/// Set the text alignment.
	#[inline]
	pub fn set_alignment(self, alignment: TextAlignment) -> Self {
		self.layout.set_alignment(alignment.to_pango());
		self
	}

	/// Set the text alignment to [`TextAlignment::Left`].
	#[inline]
	pub fn align_left(self,) -> Self {
		self.set_alignment(TextAlignment::Left)
	}

	/// Set the text alignment to [`TextAlignment::Center`].
	#[inline]
	pub fn align_center(self) -> Self {
		self.set_alignment(TextAlignment::Center)
	}

	/// Set the text alignment to [`TextAlignment::Right`].
	#[inline]
	pub fn align_right(self) -> Self {
		self.set_alignment(TextAlignment::Right)
	}

	/// Enable or disable text justification.
	#[inline]
	pub fn set_justify(self, justify: bool) -> Self {
		self.layout.set_justify(justify);
		self
	}

	/// Set the line height relative to the font size.
	#[inline]
	pub fn set_line_height(self, line_height: f64) -> Self {
		let font = self.layout.font_description().unwrap_or_default();
		assert!(font.is_size_absolute());
		let size = Length::from_device_units(font.size());
		let spacing = size * (line_height - 1.0);
		self.layout.set_spacing(spacing.as_device_units());
		self
	}

	/// Set the maximum width of the text box.
	#[inline]
	pub fn set_max_width(self, width: Option<Length>) -> Self {
		self._set_max_width(width);
		self
	}

	/// Set the maximum width of the text box.
	#[inline]
	fn _set_max_width(&self, width: Option<Length>) {
		if let Some(width) = width {
			self.layout.set_width(width.as_device_units());
		} else {
			self.layout.set_width(-1);
		}
	}

	/// Get the maximum width of the text box.
	#[inline]
	pub fn max_width(&self) -> Option<Length> {
		let max_width = self.layout.width();
		if max_width == -1 {
			None
		} else {
			Some(device_units(max_width))
		}
	}

	/// Compute the size of the text box based on the current configuration.
	#[inline]
	pub fn compute_size(&self) -> Vector2 {
		let (_absolute, logical) = self.layout.extents();
		let width = Length::from_device_units(logical.width);
		let height = Length::from_device_units(logical.height);
		Vector2::new(width, height)
	}

	/// Compute the width of the text box based on the current configuration.
	#[inline]
	pub fn compute_width(&self) -> Length {
		self.compute_size().x
	}

	/// Compute the height of the text box based on the current configuration.
	#[inline]
	pub fn compute_height(&self) -> Length {
		self.compute_size().y
	}

	/// Compute the distance of the baseline from the top of the text box.
	#[inline]
	pub fn compute_baseline(&self) -> Length {
		Length::from_device_units(self.layout.baseline())
	}

	/// Compute the natural width of the text.
	///
	/// The natural width is the computed width when no limit is applied.
	#[inline]
	pub fn compute_natural_width(&self) -> Length {
		let max_width = self.layout.width();
		self.layout.set_width(-1);
		let natural_width = self.compute_width();
		self.layout.set_width(max_width);
		natural_width
	}
}

impl Drawable for TextBox {
	fn draw(&self, surface: &Surface, position: Vector2) {
		let (_absolute, logical) = self.layout.extents();
		let offset = Vector2::new(device_units(logical.x), device_units(logical.y));
		let position = position - offset;
		surface.cairo.save().unwrap();
		self.color.set_as_source(&surface.cairo);
		surface.cairo.move_to(position.x.as_pt(), position.y.as_pt());
		pangocairo::show_layout(&surface.cairo, &self.layout);
		surface.cairo.restore().unwrap();
	}

	fn min_width(&self) -> Length {
		Length::zero()
	}

	#[inline]
	fn max_width(&self) -> Option<Length> {
		self.max_width()
	}

	#[inline]
	fn compute_size(&self) -> Vector2 {
		self.compute_size()
	}

	#[inline]
	fn compute_baseline(&self) -> Option<Length> {
		Some(self.compute_baseline())
	}

	#[inline]
	fn compute_natural_width(&self) -> Length {
		self.compute_natural_width()
	}
}

impl DrawableMut for TextBox {
	#[inline]
	fn set_max_width(&mut self, width: Option<Length>) {
		self._set_max_width(width);
	}
}

impl From<TextBox> for Box<dyn DrawableMut> {
	fn from(other: TextBox) -> Self {
		Box::new(other)
	}
}

#[derive(Debug, Clone)]
pub struct TextStyle {
	pub font: FontSpec,
	pub alignment: TextAlignment,
	pub justify: bool,
	pub line_height: f64,
}

impl Default for TextStyle {
	fn default() -> Self {
		Self {
			font: FontSpec::default(),
			alignment: TextAlignment::default(),
			justify: false,
			line_height: 1.0,
		}
	}
}

impl TextStyle {
	pub(crate) fn apply_to_layout(&self, layout: &pango::Layout) {
		let font = self.font.to_pango();
		layout.set_font_description(Some(&font));
		layout.set_alignment(self.alignment.to_pango());
		layout.set_justify(self.justify);

		let spacing = self.font.size * (self.line_height - 1.0);
		layout.set_spacing(spacing.as_device_units());
	}
}


#[derive(Debug, Copy, Clone)]
pub enum TextAlignment {
	Left,
	Center,
	Right,
}

impl TextAlignment {
	pub(crate) fn to_pango(self) -> pango::Alignment {
		match self {
			Self::Left => pango::Alignment::Left,
			Self::Center => pango::Alignment::Center,
			Self::Right => pango::Alignment::Right,
		}
	}
}

impl std::default::Default for TextAlignment {
	fn default() -> Self {
		Self::Left
	}
}

#[derive(Debug, Clone)]
pub struct FontSpec {
	pub family: String,
	pub size: Length,
	pub weight: FontWeight,
	pub style: FontStyle,
}

impl FontSpec {
	pub fn new(family: impl Into<String>, size: Length, weight: FontWeight, style: FontStyle) -> Self {
		let family = family.into();
		Self {
			family,
			size,
			weight,
			style,
		}
	}

	pub fn plain(family: impl Into<String>, size: Length) -> Self {
		Self::new(
			family,
			size,
			FontWeight::Normal,
			FontStyle::Normal,
		)
	}

	pub fn bold(family: impl Into<String>, size: Length) -> Self {
		Self::new(
			family,
			size,
			FontWeight::Bold,
			FontStyle::Normal,
		)
	}

	pub(crate) fn to_pango(&self) -> pango::FontDescription {
		let mut font = pango::FontDescription::new();
		font.set_family(&self.family);
		font.set_weight(self.weight.to_pango());
		font.set_style(self.style.to_pango());
		font.set_absolute_size(self.size.as_device_units_f64());
		font
	}
}

impl Default for FontSpec {
	fn default() -> Self {
		Self {
			family: "serif".into(),
			size: Length::from_pt(10.0),
			weight: Default::default(),
			style: Default::default(),
		}
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

impl FontWeight {
	pub(crate) fn to_pango(self) -> pango::Weight {
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

impl FontStyle {
	pub(crate) fn to_pango(self) -> pango::Style {
		match self {
			Self::Normal => pango::Style::Normal,
			Self::Oblique => pango::Style::Oblique,
			Self::Italic => pango::Style::Italic,
		}
	}
}

impl std::default::Default for FontStyle {
	fn default() -> Self {
		Self::Normal
	}
}
