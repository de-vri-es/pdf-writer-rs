const MM_PER_INCH: f64 = 25.4;
const PT_PER_INCH: f64 = 72.0;
const MM_PER_PT: f64 = MM_PER_INCH / PT_PER_INCH;
const DEVICE_PER_PT: f64 = 1024.0;

/// A length with known unit.
///
/// Internally, all values are stored as millimeters.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Length {
	mm: f64,
}

impl Length {
	/// Create a length of zero.
	pub const fn zero() -> Self {
		Self::from_mm(0.0)
	}

	/// Create a length from a value in centimeters.
	pub fn from_cm(value: f64) -> Self {
		Self::from_mm(value / 10.0)
	}

	/// Create a length from a value in millimeters.
	pub const fn from_mm(value: f64) -> Self {
		Self { mm: value }
	}

	/// Create a length from a value in inch.
	pub fn from_inch(value: f64) -> Self {
		Self::from_mm(value * MM_PER_INCH)
	}

	/// Create a length from a value in points (1/72 inch).
	pub fn from_pt(value: f64) -> Self {
		Self::from_mm(value * MM_PER_PT)
	}

	/// Create a length from cairo/pango device units (1/1024 point).
	pub(crate) fn from_device_units_f64(value: f64) -> Self {
		Self::from_pt(value / DEVICE_PER_PT)
	}

	/// Create a length from cairo/pango device units (1/1024 point).
	pub(crate) fn from_device_units(value: i32) -> Self {
		Self::from_device_units_f64(value.into())
	}

	/// Get the length as [`f64`] in centimeters.
	pub fn as_cm(self) -> f64 {
		self.as_mm() * 10.0
	}

	/// Get the length as [`f64`] in millimeters.
	pub const fn as_mm(self) -> f64 {
		self.mm
	}

	/// Get the length as [`f64`] in inches.
	pub fn as_inch(self) -> f64 {
		self.as_mm() / MM_PER_INCH
	}

	/// Get the length as [`f64`] in points (1/72 inch).
	pub fn as_pt(self) -> f64 {
		self.as_mm() / MM_PER_PT
	}

	/// Get the length as [`f64`] in cairo/pango device units (1/1024 point).
	pub(crate) fn as_device_units_f64(self) -> f64 {
		self.as_pt() * DEVICE_PER_PT
	}

	/// Get the length rounded to the nearest [`i32`] in cairo/pango device units (1/1024 point).
	pub(crate) fn as_device_units(self) -> i32 {
		self.as_device_units_f64().round() as i32
	}

	/// Get the lowest value of `self` and `other`.
	pub fn min(self, other: Self) -> Self {
		if other < self {
			other
		} else {
			self
		}
	}

	/// Get the highest value of `self` and `other`.
	pub fn max(self, other: Self) -> Self {
		if other > self {
			other
		} else {
			self
		}
	}
}

pub trait IntoLength {
	/// Interpret the value as length in centimeters.
	fn cm(self) -> Length;

	/// Interpret the value as length in millimeters.
	fn mm(self) -> Length;

	/// Interpret the value as length in inch.
	fn inch(self) -> Length;

	/// Interpret the value as length in points (1/72 inch).
	fn pt(self) -> Length;
}

impl IntoLength for f64 {
	fn cm(self) -> Length {
		Length::from_cm(self)
	}

	fn mm(self) -> Length {
		Length::from_mm(self)
	}

	fn inch(self) -> Length {
		Length::from_inch(self)
	}

	fn pt(self) -> Length {
		Length::from_pt(self)
	}
}

impl IntoLength for i32 {
	fn cm(self) -> Length {
		Length::from_cm(self.into())
	}

	fn mm(self) -> Length {
		Length::from_mm(self.into())
	}

	fn inch(self) -> Length {
		Length::from_inch(self.into())
	}

	fn pt(self) -> Length {
		Length::from_pt(self.into())
	}
}

pub(crate) fn device_units(value: i32) -> Length {
	Length::from_device_units(value)
}

impl std::ops::Neg for Length {
	type Output = Length;

	#[inline]
	fn neg(self) -> Self::Output {
		Length::from_mm(-self.mm)
	}
}

impl std::ops::Add for Length {
	type Output = Length;

	#[inline]
	fn add(self, rhs: Length) -> Self::Output {
		Length::from_mm(self.as_mm() + rhs.as_mm())
	}
}

impl std::ops::AddAssign for Length {
	#[inline]
	fn add_assign(&mut self, rhs: Length) {
		self.mm += rhs.as_mm()
	}
}

impl std::ops::Sub for Length {
	type Output = Length;

	#[inline]
	fn sub(self, rhs: Length) -> Self::Output {
		Length::from_mm(self.as_mm() - rhs.as_mm())
	}
}

impl std::ops::SubAssign for Length {
	#[inline]
	fn sub_assign(&mut self, rhs: Length) {
		self.mm -= rhs.as_mm()
	}
}

impl std::ops::Mul<f64> for Length {
	type Output = Length;

	#[inline]
	fn mul(self, rhs: f64) -> Self::Output {
		Length::from_mm(self.as_mm() * rhs)
	}
}

impl std::ops::Mul<Length> for f64 {
	type Output = Length;

	#[inline]
	fn mul(self, rhs: Length) -> Self::Output {
		rhs * self
	}
}

impl std::ops::MulAssign<f64> for Length {
	fn mul_assign(&mut self, rhs: f64) {
		self.mm *= rhs;
	}
}

impl std::ops::Div<f64> for Length {
	type Output = Length;

	#[inline]
	fn div(self, rhs: f64) -> Self::Output {
		Length::from_mm(self.as_mm() / rhs)
	}
}

impl std::ops::DivAssign<f64> for Length {
	fn div_assign(&mut self, rhs: f64) {
		self.mm /= rhs;
	}
}

impl std::ops::Div<Length> for Length {
	type Output = f64;

	#[inline]
	fn div(self, rhs: Length) -> Self::Output {
		self.as_mm() / rhs.as_mm()
	}
}

impl std::iter::Sum<Length> for Length {
	fn sum<I: Iterator<Item = Length>>(iter: I) -> Self
	{
		let mut total = Self::zero();
		for value in iter {
			total += value;
		}
		total
	}
}

impl<'a> std::iter::Sum<&'a Length> for Length {
	fn sum<I: Iterator<Item = &'a Length>>(iter: I) -> Self
	{
		let mut total = Self::zero();
		for value in iter {
			total += *value;
		}
		total
	}
}
