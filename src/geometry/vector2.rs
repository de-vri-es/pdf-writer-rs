use super::Length;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector2 {
	pub x: Length,
	pub y: Length,
}

impl Vector2 {
	pub const fn new(x: Length, y: Length) -> Self {
		Self { x, y }
	}

	pub const fn zero() -> Self {
		Self::new(Length::from_mm(0.0), Length::from_mm(0.0))
	}

	pub fn norm_squared(self) -> Length {
		let x = self.x.as_mm();
		let y = self.y.as_mm();
		Length::from_mm(x*x + y*y)
	}

	pub fn norm(self) -> Length {
		Length::from_mm(self.norm_squared().as_mm().sqrt())
	}
}

impl std::ops::Neg for Vector2 {
	type Output = Vector2;

	#[inline]
	fn neg(self) -> Self::Output {
		Vector2::new(-self.x, -self.y)
	}
}

impl std::ops::Add for Vector2 {
	type Output = Vector2;

	#[inline]
	fn add(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl std::ops::AddAssign for Vector2 {
	#[inline]
	fn add_assign(&mut self, rhs: Vector2) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl std::ops::Sub for Vector2 {
	type Output = Vector2;

	#[inline]
	fn sub(self, rhs: Vector2) -> Self::Output {
		Vector2::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl std::ops::SubAssign for Vector2 {
	#[inline]
	fn sub_assign(&mut self, rhs: Vector2) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl std::ops::Mul<f64> for Vector2 {
	type Output = Vector2;

	#[inline]
	fn mul(self, rhs: f64) -> Self::Output {
		Vector2::new(self.x * rhs, self.y * rhs)
	}
}

impl std::ops::Mul<Vector2> for f64 {
	type Output = Vector2;

	#[inline]
	fn mul(self, rhs: Vector2) -> Self::Output {
		rhs * self
	}
}

impl std::ops::MulAssign<f64> for Vector2 {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl std::ops::Div<f64> for Vector2 {
	type Output = Vector2;

	#[inline]
	fn div(self, rhs: f64) -> Self::Output {
		Vector2::new(self.x / rhs, self.y / rhs)
	}
}

impl std::ops::DivAssign<f64> for Vector2 {
	fn div_assign(&mut self, rhs: f64) {
		self.x /= rhs;
		self.y /= rhs;
	}
}
