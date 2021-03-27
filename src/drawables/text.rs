use crate::{Context, Drawable, Length, Surface, Vector2, device_units};

pub struct Text {
	layout: pango::Layout,
}

impl Text {
	pub fn new(context: &Context) -> Self {
		let layout = pango::Layout::new(&context.pango);
		TextStyle::default().apply_to_layout(&layout);
		Self { layout }
	}

	pub fn set_text(&mut self, text: &str) -> &mut Self {
		self.layout.set_text(text);
		self
	}

	pub fn set_style(&mut self, style: &TextStyle) -> &mut Self {
		style.apply_to_layout(&self.layout);
		self
	}

	pub fn set_font(&mut self, font: &FontSpec) -> &mut Self {
		self.layout.set_font_description(Some(&font.to_pango()));
		self
	}

	pub fn set_font_family(&mut self, family: &str) -> &mut Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.get_font_description().unwrap();
		font.set_family(family);
		self.layout.set_font_description(Some(&font));
		self
	}

	pub fn set_font_size(&mut self, size: Length) -> &mut Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.get_font_description().unwrap();
		font.set_absolute_size(size.as_device_units_f64());
		self.layout.set_font_description(Some(&font));
		self
	}

	pub fn set_font_weight(&mut self, weight: FontWeight) -> &mut Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.get_font_description().unwrap();
		font.set_weight(weight.to_pango());
		self.layout.set_font_description(Some(&font));
		self
	}

	pub fn make_bold(&mut self) -> &mut Self {
		self.set_font_weight(FontWeight::Bold)
	}


	pub fn make_thin(&mut self) -> &mut Self {
		self.set_font_weight(FontWeight::Thin)
	}

	pub fn set_font_style(&mut self, style: FontStyle) -> &mut Self {
		// We always set a font, so unwrap should never fail.
		let mut font = self.layout.get_font_description().unwrap();
		font.set_style(style.to_pango());
		self.layout.set_font_description(Some(&font));
		self
	}

	pub fn make_italic(&mut self) -> &mut Self {
		self.set_font_style(FontStyle::Italic)
	}

	pub fn make_oblique(&mut self) -> &mut Self {
		self.set_font_style(FontStyle::Oblique)
	}

	pub fn set_alignment(&mut self, alignment: TextAlignment) -> &mut Self {
		self.layout.set_alignment(alignment.to_pango());
		self
	}

	pub fn align_left(&mut self,) -> &mut Self {
		self.set_alignment(TextAlignment::Left)
	}

	pub fn align_center(&mut self) -> &mut Self {
		self.set_alignment(TextAlignment::Center)
	}

	pub fn align_right(&mut self) -> &mut Self {
		self.set_alignment(TextAlignment::Right)
	}

	pub fn set_justify(&mut self, justify: bool) -> &mut Self {
		self.layout.set_justify(justify);
		self
	}

	pub fn set_line_height(&mut self, line_height: f64) -> &mut Self {
		let font = self.layout.get_font_description().unwrap_or_default();
		assert!(font.get_size_is_absolute());
		let size = Length::from_device_units(font.get_size());
		let spacing = size * (line_height - 1.0);
		self.layout.set_spacing(spacing.as_device_units());
		self
	}

	pub fn set_max_width(&mut self, width: Option<Length>) -> &mut Self {
		if let Some(width) = width {
			self.layout.set_width(width.as_device_units());
		} else {
			self.layout.set_width(-1);
		}
		self
	}

	pub fn get_max_width(&self) -> Option<Length> {
		let max_width = self.layout.get_width();
		if max_width == -1 {
			None
		} else {
			Some(device_units(max_width))
		}
	}

	pub fn size(&self) -> Vector2 {
		let (_absolute, logical) = self.layout.get_extents();
		let width = Length::from_device_units(logical.width);
		let height = Length::from_device_units(logical.height);
		Vector2::new(width, height)
	}

	pub fn width(&self) -> Length {
		self.size().x
	}

	pub fn height(&self) -> Length {
		self.size().y
	}

	pub fn draw(&self, surface: impl AsRef<Surface>, position: Vector2) {
		let surface = surface.as_ref();
		let (_absolute, logical) = self.layout.get_extents();
		let offset = Vector2::new(device_units(logical.x), device_units(logical.y));
		let position = position - offset;
		surface.cairo.move_to(position.x.as_pt(), position.y.as_pt());
		pangocairo::show_layout(&surface.cairo, &self.layout);
	}
}

impl Drawable for Text {
	fn draw(&self, surface: &Surface, position: Vector2) {
		self.draw(surface, position);
	}

	fn set_max_width(&mut self, width: Option<Length>) {
		self.set_max_width(width);
	}

	fn get_max_width(&self) -> Option<Length> {
		self.get_max_width()
	}

	fn size(&self) -> Vector2 {
		self.size()
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
