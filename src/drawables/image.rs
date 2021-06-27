use crate::{
	Drawable,
	DrawableMut,
	Length,
	Surface,
	Vector2,
};

pub enum ScaleMode {
	Fit,
	Stretch,
}

pub struct Image {
	image: cairo::ImageSurface,
	width: Option<Length>,
	height: Option<Length>,
	scale_mode: ScaleMode,
}

fn to_cairo(image: image::DynamicImage) -> Result<cairo::ImageSurface, cairo::Error> {
	let image = image.to_bgra8();
	let width = image.width();
	let height = image.height();
	let data = image.into_vec();
	cairo::ImageSurface::create_for_data(data, cairo::Format::ARgb32, width as i32, height as i32, width as i32 * 4)
}

impl Image {
	pub fn new(image: image::DynamicImage) -> Result<Self, String> {
		Ok(Self {
			image: to_cairo(image).map_err(|e| e.to_string())?,
			width: None,
			height: None,
			scale_mode: ScaleMode::Fit,
		})
	}

	pub fn set_width(mut self, width: impl Into<Option<Length>>) -> Self {
		self.width = width.into();
		self
	}

	pub fn set_height(mut self, height: impl Into<Option<Length>>) -> Self {
		self.height = height.into();
		self
	}

	pub fn set_scale_mode(mut self, scale_mode: ScaleMode) -> Self {
		self.scale_mode = scale_mode;
		self
	}

	pub fn fit(mut self) -> Self {
		self.scale_mode = ScaleMode::Fit;
		self
	}

	pub fn stretch(mut self) -> Self {
		self.scale_mode = ScaleMode::Stretch;
		self
	}

	pub fn image_size(&self) -> Vector2 {
		Vector2::new(
			Length::from_pt(self.image.get_width().into()),
			Length::from_pt(self.image.get_height().into()),
		)
	}

	pub fn compute_size(&self) -> Vector2 {
		let image_size = self.image_size();
		match (self.width, self.height) {
			(None, None) => image_size,
			(Some(width), None) => (width / image_size.x) * image_size,
			(None, Some(height)) => (height / image_size.y) * image_size,
			(Some(width), Some(height)) => match self.scale_mode {
				ScaleMode::Stretch => Vector2::new(width, height),
				ScaleMode::Fit => {
					let scale = (width / image_size.x).min(height / image_size.y);
					scale * image_size
				},
			}
		}
	}

	pub fn draw(&self, surface: &Surface, position: Vector2) {
		let image_size = self.image_size();
		let target_size = self.compute_size();

		let mut matrix = cairo::Matrix::identity();
		matrix.scale(image_size.x / target_size.x, image_size.y / target_size.y);
		matrix.translate(-position.x.as_pt(), -position.y.as_pt());

		surface.cairo.save();
		surface.cairo.set_source_surface(&self.image, 0.0, 0.0);
		surface.cairo.get_source().set_matrix(matrix);
		surface.cairo.rectangle(position.x.as_pt(), position.y.as_pt(), target_size.x.as_pt(), target_size.y.as_pt());
		surface.cairo.fill();
		surface.cairo.restore();
	}
}

impl Drawable for Image {
	fn draw(&self, surface: &Surface, position: Vector2) {
		self.draw(surface, position)
	}

	fn min_width(&self) -> Length {
		Length::zero()
	}

	#[inline]
	fn max_width(&self) -> Option<Length> {
		self.width
	}

	#[inline]
	fn compute_size(&self) -> Vector2 {
		self.compute_size()
	}

	#[inline]
	fn compute_baseline(&self) -> Option<Length> {
		None
	}

	#[inline]
	fn compute_natural_width(&self) -> Length {
		Length::from_pt(self.image.get_width().into())
	}
}

impl DrawableMut for Image {
	#[inline]
	fn set_max_width(&mut self, width: Option<Length>) {
		self.width = width;
	}
}
