
pub struct Mm {
	_private: (),
}

pub struct Pt {
	_private: (),
}

pub(crate) struct PangoUnit {
	_private: (),
}

pub const MM_PER_PT: euclid::Scale<f64, Pt, Mm> = euclid::Scale::new(25.4 / 72.0);
pub const PT_PER_MM: euclid::Scale<f64, Mm, Pt> = euclid::Scale::new(72.0 / 25.4);
pub(crate) const PANGO_PER_PT: euclid::Scale<f64, Pt, PangoUnit> = euclid::Scale::new(1024.0);
pub(crate) const PT_PER_PANGO: euclid::Scale<f64, PangoUnit, Pt> = euclid::Scale::new(1.0 / 1024.0);

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

/// The logical and absolute extent of a text box.
#[derive(Debug, Clone)]
pub struct TextExtent {
	pub logical: Box2<Mm>,
	pub absolute: Box2<Mm>,
}

pub(crate) fn box_from_pango(rect: pango::Rectangle) -> Box2<Pt> {
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
