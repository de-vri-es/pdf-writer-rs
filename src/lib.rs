mod geometry;
pub use geometry::*;

mod drawables;
pub use drawables::*;

mod pdf;
pub use pdf::*;

pub const A4: Vector2 = Vector2::new(Length::from_mm(210.0), Length::from_mm(297.0));

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

/// Drawable item.
///
/// All drawables are modeled as items that can be limited at a maximum width.
/// Items will automatically grow in height as needed, to compensate for lost width.
pub trait Drawable {
	/// Draw the item to a surface at a given position.
	fn draw(&self, surface: &Surface, position: Vector2);

	/// Get the minimum width of the item.
	fn min_width(&self) -> Length;

	/// Get the maximum width of the item.
	fn max_width(&self) -> Option<Length>;

	/// Compute the size of the item for the current configuration.
	fn compute_size(&self) -> Vector2;

	/// Compute the width of the item for the current configuration.
	///
	/// This is the same as the `x` component returned by [`Drawable::compute_size`],
	/// but it may be more efficient because the height does not need to be computed.
	fn compute_width(&self) -> Length {
		self.compute_size().x
	}

	/// Compute the height of the item for the current configuration.
	///
	/// This is the same as the `x` component returned by [`Drawable::compute_size`],
	/// but it may be more efficient because the width does not need to be computed.
	/// (Although most items will need to compute their width in order to compute the height.)
	fn compute_height(&self) -> Length {
		self.compute_size().y
	}

	/// Compute the distance of the baseline from the top of the drawable.
	///
	/// If the drawable has no baseline (because it has no text), this should return None.
	fn compute_baseline(&self) -> Option<Length>;

	/// Compute the natural width of the item.
	///
	/// This is the computed width of the item if no width limit is applied.
	fn compute_natural_width(&self) -> Length;

	/// Add a position offset to the the drawable.
	fn offset(self, offset: Vector2) -> Offset<Self>
	where
		Self: Sized
	{
		Offset::new(self, offset)
	}
}

/// Drawable item (mutable interface).
///
/// All drawables are modeled as items that can be limited at a maximum width.
/// Items will automatically grow in height as needed, to compensate for lost width.
pub trait DrawableMut: Drawable {
	/// Set the maximum width of the item.
	fn set_max_width(&mut self, width: Option<Length>);
}

pub struct HighlightContext {
	syntax_set: syntect::parsing::SyntaxSet,
	themes: syntect::highlighting::ThemeSet,
	default_theme: String,
}

impl HighlightContext {
	fn new() -> Self {
		let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
		let themes = syntect::highlighting::ThemeSet::load_defaults();
		let default_theme = themes.themes.keys().next().unwrap().clone();
		Self {
			syntax_set,
			themes,
			default_theme,
		}
	}

	pub fn load_themes(&mut self, directory: impl AsRef<std::path::Path>) -> Result<(), String> {
		self.themes.add_from_folder(directory).map_err(|e| e.to_string())
	}

	pub fn set_default_theme(&mut self, theme: &str) -> Result<(), String> {
		if self.themes.themes.get(theme).is_some() {
			self.default_theme = theme.to_string();
			Ok(())
		} else {
			Err(format!("unknown theme: {}", theme))
		}
	}

	pub fn themes(&self) -> impl Iterator<Item = &str> {
		self.themes.themes.keys().map(|x| x.as_str())
	}
}

/// A context to create PDFs.
pub struct Context {
	pango: pango::Context,
	fake_pdf: cairo::PdfSurface,
	highlighting: HighlightContext,
}

/// A surface to draw [`Drawable`] items on.
pub struct Surface {
	cairo: cairo::Context,
	size: Vector2,
}

impl Context {
	/// Create a new context.
	pub fn new() -> Result<Self, String> {
		let pango = pango::Context::new();

		let font_map = pangocairo::FontMap::default()
			.ok_or("failed to get default font map")?;
		pango.set_font_map(&font_map);
		pango.load_font(&FontSpec::default().to_pango()).unwrap();

		let fake_pdf = cairo::PdfSurface::for_stream(100.0, 100.0, Vec::new())
			.map_err(|e| format!("failed to create PDF surface: {}", e))?;

		Ok(Self {
			pango,
			fake_pdf,
			highlighting: HighlightContext::new(),
		})
	}

	pub fn highlighting(&self) -> &HighlightContext {
		&self.highlighting
	}

	pub fn highlighting_mut(&mut self) -> &mut HighlightContext {
		&mut self.highlighting
	}

	/// Create a new PDF backed by a [`Write`] stream.
	pub fn pdf<W: std::io::Write + 'static>(&self, stream: W) -> Result<PdfWriter, String> {
		PdfWriter::new(stream)
	}

	/// Create a new page.
	pub fn page(&self) -> Result<Page, String> {
		Page::new(self)
	}

	/// Create a new text box.
	pub fn text_box(&self) -> TextBox {
		TextBox::new(self)
	}

	/// Create a new item list.
	pub fn item_list(&self, bullet_font: &FontSpec) -> ItemList {
		ItemList::new(self, bullet_font)
	}
}

impl Surface {
	/// Wrap a cairo surface.
	fn new(surface: &cairo::Surface, size: Vector2) -> Self {
		Self {
			cairo: cairo::Context::new(surface).unwrap(),
			size,
		}
	}
}

impl AsRef<Surface> for Surface {
	fn as_ref(&self) -> &Surface {
		self
	}
}

impl<T: Drawable> Drawable for &'_ T {
	fn draw(&self, surface: &Surface, position: Vector2) {
		T::draw(self, surface, position)
	}

	fn min_width(&self) -> Length {
		T::min_width(self)
	}

	fn max_width(&self) -> Option<Length> {
		T::max_width(self)
	}

	fn compute_size(&self) -> Vector2 {
		T::compute_size(self)
	}

	fn compute_width(&self) -> Length {
		T::compute_width(self)
	}

	fn compute_height(&self) -> Length {
		T::compute_height(self)
	}

	fn compute_baseline(&self) -> Option<Length> {
		T::compute_baseline(self)
	}

	fn compute_natural_width(&self) -> Length {
		T::compute_natural_width(self)
	}
}

impl<T: Drawable> Drawable for &'_ mut T {
	fn draw(&self, surface: &Surface, position: Vector2) {
		T::draw(self, surface, position)
	}

	fn min_width(&self) -> Length {
		T::min_width(self)
	}

	fn max_width(&self) -> Option<Length> {
		T::max_width(self)
	}

	fn compute_size(&self) -> Vector2 {
		T::compute_size(self)
	}

	fn compute_width(&self) -> Length {
		T::compute_width(self)
	}

	fn compute_height(&self) -> Length {
		T::compute_height(self)
	}

	fn compute_baseline(&self) -> Option<Length> {
		T::compute_baseline(self)
	}

	fn compute_natural_width(&self) -> Length {
		T::compute_natural_width(self)
	}
}

impl<T: DrawableMut> DrawableMut for &'_ mut T {
	fn set_max_width(&mut self, width: Option<Length>) {
		T::set_max_width(self, width)
	}
}
