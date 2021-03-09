use crate::{Length, Pt};

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

	pub(crate) fn to_pango(&self) -> pango::FontDescription {
		let mut font = pango::FontDescription::new();
		font.set_family(self.family);
		font.set_weight(self.weight.to_pango());
		font.set_style(self.style.to_pango());
		font.set_absolute_size((self.size * crate::PANGO_PER_PT).get());
		font
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
