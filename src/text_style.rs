use crate::{
	FontSpec,
	Length,
	Mm,
	Point2,
	Size2,
	Vector2,
};

#[derive(Debug, Clone)]
pub struct TextStyle {
	pub font: FontSpec,
	pub align: TextAlign,
	pub justify: bool,
	pub line_height: f64,
}

impl TextStyle {
	pub(crate) fn apply_to_layout(&self, layout: &pango::Layout) {
		let font = self.font.to_pango();
		layout.set_font_description(Some(&font));
		layout.set_alignment(self.align.to_pango());
		layout.set_justify(self.justify);

		let spacing = self.font.size * (self.line_height - 1.0);
		layout.set_spacing((spacing * crate::PANGO_PER_PT).get().round() as i32);
	}
}


#[derive(Debug, Copy, Clone)]
pub enum TextAlign {
	Left,
	Center,
	Right,
}

impl TextAlign {
	pub(crate) fn to_pango(self) -> pango::Alignment {
		match self {
			Self::Left => pango::Alignment::Left,
			Self::Center => pango::Alignment::Center,
			Self::Right => pango::Alignment::Right,
		}
	}
}

impl std::default::Default for TextAlign {
	fn default() -> Self {
		Self::Left
	}
}

#[derive(Debug, Clone)]
pub struct BoxPosition {
	pub point: Point2<Mm>,
	pub anchor_h: HorizontalAnchor,
	pub anchor_v: VerticalAnchor,
}

impl BoxPosition {
	pub fn new(point: Point2<Mm>, anchor_h: HorizontalAnchor, anchor_v: VerticalAnchor) -> Self {
		Self { point, anchor_h, anchor_v }
	}

	pub fn at(point: Point2<Mm>) -> Self {
		Self::new(point, HorizontalAnchor::Left, VerticalAnchor::Top)
	}

	pub fn at_xy(x: Length<Mm>, y: Length<Mm>) -> Self {
		Self::at(Point2::new(x.get(), y.get()))
	}

	pub(crate) fn alignment_offset<Unit>(&self, size: Size2<Unit>, baseline: Length<Unit>) -> Vector2<Unit> {
		let x = match self.anchor_h {
			HorizontalAnchor::Left => 0.0,
			HorizontalAnchor::Middle => size.width * -0.5,
			HorizontalAnchor::Right => size.width * -1.0,
		};

		let y = match self.anchor_v {
			VerticalAnchor::Top => 0.0,
			VerticalAnchor::Baseline => -baseline.get(),
			VerticalAnchor::Middle => size.height * -0.5,
			VerticalAnchor::Bottom => size.height * -1.0,
		};

		Vector2::new(x, y)
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
