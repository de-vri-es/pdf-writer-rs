const MM_PER_INCH: f64 = 25.4;
const PT_PER_INCH: f64 = 72.0;
const MM_PER_PT: f64 = MM_PER_INCH / PT_PER_INCH;
const DEVICE_PER_PT: f64 = 1024.0;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length {
	mm: f64,
}

impl Length {
	pub fn from_cm(value: f64) -> Self {
		Self::from_mm(value / 10.0)
	}

	pub fn from_mm(value: f64) -> Self {
		Self { mm: value }
	}

	pub fn from_inch(value: f64) -> Self {
		Self::from_mm(value * MM_PER_INCH)
	}

	pub fn from_pt(value: f64) -> Self {
		Self::from_mm(value * MM_PER_PT)
	}

	pub(crate) fn from_device_units_f64(value: f64) -> Self {
		Self::from_pt(value / DEVICE_PER_PT)
	}

	pub(crate) fn from_device_units(value: i32) -> Self {
		Self::from_device_units_f64(value.into())
	}

	pub fn as_cm(self) -> f64 {
		self.as_mm() * 10.0
	}

	pub fn as_mm(self) -> f64 {
		self.mm
	}

	pub fn as_inch(self) -> f64 {
		self.as_mm() / MM_PER_INCH
	}

	pub fn as_pt(self) -> f64 {
		self.as_mm() / MM_PER_PT
	}

	pub(crate) fn as_device_units_f64(self) -> f64 {
		self.as_pt() * DEVICE_PER_PT
	}

	pub(crate) fn as_device_units(self) -> i32 {
		self.as_device_units_f64().round() as i32
	}
}

pub fn mm(value: f64) -> Length {
	Length::from_mm(value)
}

pub fn cm(value: f64) -> Length {
	Length::from_cm(value)
}

pub fn pt(value: f64) -> Length {
	Length::from_pt(value)
}

pub fn inch(value: f64) -> Length {
	Length::from_inch(value)
}

pub(crate) fn device_units(value: i32) -> Length {
	Length::from_device_units(value)
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
