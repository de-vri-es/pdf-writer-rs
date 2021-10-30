use crate::{Color, Drawable, DrawableMut, Length, Surface, Vector2};

pub struct SourceCode<'a> {
	context: &'a crate::Context,
	code: &'a str,
	syntax: &'a syntect::parsing::SyntaxReference,
	theme: &'a syntect::highlighting::Theme,
	font: crate::FontSpec,
	background: Option<Color>,
	foreground: Option<Color>,
}

impl<'a> SourceCode<'a> {
	pub fn new(context: &'a crate::Context, languague: &str) -> Result<Self, String> {
		let syntax = context.highlighting.syntax_set.find_syntax_by_token(languague)
			.ok_or_else(|| format!("unknown language: {}", languague))?;
		Ok(Self {
			context,
			code: "",
			syntax,
			theme: context.highlighting.themes.themes.get(&context.highlighting.default_theme).unwrap(),
			font: crate::FontSpec::plain("monospace", Length::from_pt(10.0)),
			background: None,
			foreground: None,
		})
	}

	pub fn set_code(mut self, code: &'a str) -> Self {
		self.code = code;
		self
	}

	pub fn set_theme(mut self, theme: &str) -> Result<Self, String> {
		self.theme = self.context.highlighting.themes.themes
			.get(theme)
			.ok_or_else(|| format!("unknown highlighting theme: {}", theme))?;
		Ok(self)
	}

	pub fn set_font(mut self, font: crate::FontSpec) -> Self {
		self.font = font;
		self
	}

	pub fn set_text_color(mut self, color: impl Into<Option<Color>>) -> Self {
		self.foreground = color.into();
		self
	}

	pub fn set_background_color(mut self, color: impl Into<Option<Color>>) -> Self {
		self.background = color.into();
		self
	}

	pub fn highlight(&self) -> HighlightedSourceCode {
		HighlightedSourceCode::new(self)
	}
}

pub struct HighlightedSourceCode {
	lines: Vec<SourceCodeLine>,
	max_width: Option<Length>,
	size_info: SourceCodeSizeInfo,
	background: Option<crate::Color>,
	foreground: Option<crate::Color>,
	line_nr_background: Option<crate::Color>,
	line_nr_foreground: Option<crate::Color>,
}

pub struct SourceCodeSizeInfo {
	line_nr_width: Length,
	source_width: Length,
	natural_source_width: Length,
	height: Length,
}

struct SourceCodeLine {
	line_nr: pango::Layout,
	source: pango::Layout,
}

impl HighlightedSourceCode {
	pub fn new(source: &SourceCode) -> Self {
		let pango_font = source.font.to_pango();

		let mut highlighter = syntect::easy::HighlightLines::new(source.syntax, source.theme);
		let mut lines = Vec::new();
		for (line_nr, line) in syntect::util::LinesWithEndings::from(source.code).enumerate() {
			let line_nr_layout = pango::Layout::new(&source.context.pango);
			line_nr_layout.set_font_description(Some(&pango_font));
			line_nr_layout.set_text(&format!("{} ", line_nr));

			let ranges = highlighter.highlight(line, &source.context.highlighting.syntax_set);
			let source_attrs = highlights_to_attrs(&ranges, source.theme);
			let source_layout = pango::Layout::new(&source.context.pango);
			source_layout.set_font_description(Some(&pango_font));
			source_layout.set_text(line.strip_suffix('\n').unwrap_or(line));
			source_layout.set_attributes(Some(&source_attrs));
			source_layout.set_indent(-(source.font.size * 2.0).as_device_units());

			lines.push(SourceCodeLine {
				line_nr: line_nr_layout,
				source: source_layout,
			});
		}

		let size_info = SourceCodeSizeInfo::compute(&lines);
		let background = source.background.or_else(|| source.theme.settings.background.map(crate::Color::from));
		let foreground = source.foreground.or_else(|| source.theme.settings.foreground.map(crate::Color::from));
		let line_nr_background = source.theme.settings.gutter.map(crate::Color::from);
		let line_nr_foreground = source.theme.settings.gutter_foreground.map(crate::Color::from);

		Self {
			max_width: None,
			lines,
			size_info,
			background,
			foreground,
			line_nr_background,
			line_nr_foreground,
		}
	}

	pub fn set_max_width(mut self, max_width: impl Into<Option<Length>>) -> Self {
		self._set_max_width(max_width.into());
		self
	}

	fn max_width(&self) -> Option<Length> {
		self.max_width
	}

	pub fn _set_max_width(&mut self, max_width: Option<Length>) {
		match max_width {
			None => {
				for line in &mut self.lines {
					line.source.set_width(-1);
				}
			},
			Some(max_width) => {
				let source_width = max_width - self.size_info.line_nr_width;
				for line in &self.lines {
					line.source.set_width(source_width.as_device_units());
				}
			}
		}
		self.size_info = SourceCodeSizeInfo::compute(&self.lines);
	}

	pub fn set_font(mut self, font_spec: crate::FontSpec) -> Self {
		let mut line_nr_width = Length::zero();
		let pango_font = font_spec.to_pango();
		for line in &self.lines {
			line.line_nr.set_font_description(Some(&pango_font));
			line.source.set_font_description(Some(&pango_font));
			line_nr_width = line_nr_width.max(Length::from_device_units(line.line_nr.extents().1.width));
		}
		self.size_info.line_nr_width = line_nr_width;
		self._set_max_width(self.max_width);
		self
	}

	pub fn compute_natural_width(&self) -> Length {
		self.size_info.line_nr_width + self.size_info.natural_source_width
	}

	pub fn compute_size(&self) -> Vector2 {
		let width = self.size_info.line_nr_width + self.size_info.source_width;
		Vector2::new(width, self.size_info.height)
	}

	fn draw(&self, surface: &Surface, position: Vector2) {
		if let Some(bg) = &self.line_nr_background {
			surface.cairo.save().unwrap();
			bg.set_as_source(&surface.cairo);
			surface.cairo.rectangle(position.x.as_pt(), position.y.as_pt(), self.size_info.line_nr_width.as_pt(), self.size_info.height.as_pt());
			surface.cairo.fill().unwrap();
			surface.cairo.restore().unwrap();
		}

		if let Some(bg) = &self.background {
			surface.cairo.save().unwrap();
			bg.set_as_source(&surface.cairo);
			surface.cairo.rectangle(
				(position.x + self.size_info.line_nr_width).as_pt() - 1.0,
				position.y.as_pt(),
				(self.size_info.source_width).as_pt() + 1.0,
				self.size_info.height.as_pt()
			);
			surface.cairo.fill().unwrap();
			surface.cairo.restore().unwrap();
		}

		let mut y = position.y;
		for line in &self.lines {
			let line_nr_extents = line.line_nr.extents().1;
			let source_extents = line.source.extents().1;
			let height = Length::from_device_units(line_nr_extents.height.max(source_extents.height));

			let x = position.x + self.size_info.line_nr_width + Length::from_device_units(line_nr_extents.x - line_nr_extents.width);
			if let Some(fg) = &self.line_nr_foreground.or(self.foreground) {
				surface.cairo.set_source_rgba(fg.red as f64 / 255.0, fg.green as f64 / 255.0, fg.blue as f64 / 255.0, fg.alpha as f64 / 255.0);
			}
			surface.cairo.move_to(x.as_pt(), y.as_pt());
			pangocairo::show_layout(&surface.cairo, &line.line_nr);

			if let Some(fg) = &self.foreground {
				surface.cairo.set_source_rgba(fg.red as f64 / 255.0, fg.green as f64 / 255.0, fg.blue as f64 / 255.0, fg.alpha as f64 / 255.0);
			}
			let x = x + Length::from_device_units(line_nr_extents.width);
			surface.cairo.move_to(x.as_pt(), y.as_pt());
			pangocairo::show_layout(&surface.cairo, &line.source);

			y += height;
		}
	}
}

impl SourceCodeSizeInfo {
	fn compute(lines: &[SourceCodeLine]) -> Self {
		let mut source_width = Length::from_cm(1.0);
		let mut natural_source_width = Length::from_cm(1.0);
		let mut line_nr_width = Length::from_mm(0.0);
		let mut height = Length::zero();

		for line in lines {
			let old_max_width = line.source.width();
			line.source.set_width(-1);
			natural_source_width = natural_source_width.max(Length::from_device_units(line.source.extents().1.width));
			line.source.set_width(old_max_width);

			let source_extents = line.source.extents().1;
			let line_nr_extents = line.line_nr.extents().1;
			source_width = source_width.max(Length::from_device_units(source_extents.width));
			line_nr_width = line_nr_width.max(Length::from_device_units(line_nr_extents.width));
			height += Length::from_device_units(source_extents.height.max(line_nr_extents.height));
		}

		Self {
			line_nr_width,
			source_width,
			natural_source_width,
			height,
		}
	}
}

fn highlights_to_attrs(ranges: &[(syntect::highlighting::Style, &str)], theme: &syntect::highlighting::Theme) -> pango::AttrList {
	let source_attrs = pango::AttrList::new();
	let mut start_index = 0;
	for (style, text) in ranges {
		if style.font_style.contains(syntect::highlighting::FontStyle::BOLD) {
			source_attrs.change(set_attr_span(pango::Attribute::new_weight(pango::Weight::Bold), start_index, text.len()))
		}
		if style.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
			source_attrs.change(set_attr_span(pango::Attribute::new_style(pango::Style::Italic), start_index, text.len()))
		}
		if style.font_style.contains(syntect::highlighting::FontStyle::UNDERLINE) {
			source_attrs.change(set_attr_span(pango::Attribute::new_underline(pango::Underline::Single), start_index, text.len()))
		}

		if theme.settings.background.map(|x| x != style.background).unwrap_or(true) {
			let syntect::highlighting::Color{r, g, b, a} = style.background;
			source_attrs.change(set_attr_span(pango::Attribute::new_background(r as u16 * 257, g as u16 * 257, b as u16 * 257), start_index, text.len()));
			if a != 255 {
				source_attrs.change(set_attr_span(pango::Attribute::new_background_alpha(a as u16 * 257), start_index, text.len()));
			}
		}

		if theme.settings.foreground.map(|x| x != style.foreground).unwrap_or(true) {
			let syntect::highlighting::Color{r, g, b, a} = style.foreground;
			source_attrs.change(set_attr_span(pango::Attribute::new_foreground(r as u16 * 257, g as u16 * 257, b as u16 * 257), start_index, text.len()));
			if a != 255 {
				source_attrs.change(set_attr_span(pango::Attribute::new_foreground_alpha(a as u16 * 257), start_index, text.len()));
			}
		}

		start_index += text.len();
	}
	source_attrs
}

fn set_attr_span(mut attribute: pango::Attribute, start_index: usize, length: usize) -> pango::Attribute {
	attribute.set_start_index(start_index as u32);
	attribute.set_end_index(start_index as u32 + length as u32);
	attribute
}

impl Drawable for HighlightedSourceCode {
	fn draw(&self, surface: &Surface, position: Vector2) {
		self.draw(surface, position)
	}

	fn min_width(&self) -> Length {
		if self.lines.is_empty() {
			Length::zero()
		} else {
			self.size_info.line_nr_width + Length::from_cm(1.0)
		}
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
		if self.lines.is_empty() {
			None
		} else {
			Some(Length::from_device_units(self.lines[0].line_nr.baseline()))
		}
	}

	#[inline]
	fn compute_natural_width(&self) -> Length {
		self.compute_natural_width()
	}
}

impl DrawableMut for HighlightedSourceCode {
	#[inline]
	fn set_max_width(&mut self, width: Option<Length>) {
		self._set_max_width(width);
	}
}

impl From<syntect::highlighting::Color> for crate::Color {
	fn from(other: syntect::highlighting::Color) -> Self {
		Self {
			red: other.r,
			green: other.g,
			blue: other.b,
			alpha: other.a,
		}
	}
}
