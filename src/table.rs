use crate::{
	BoxPosition,
	Length,
	Mm,
	Page,
	PdfWriter,
	Point2,
	Pt,
	PT_PER_MM,
	Size2,
	TextBox,
	TextStyle,
	mm,
};

pub struct Table {
	columns: usize,
	cells: Vec<TextBox>,
	size: Size2<Mm>,
	position: BoxPosition,
}

pub struct TableCell<'a> {
	text: &'a str,
	style: &'a TextStyle,
}

impl<'a> TableCell<'a> {
	fn new(text: &'a str, style: &'a TextStyle) -> Self {
		Self { text, style }
	}
}

pub fn cell<'a>(text: &'a str, style: &'a TextStyle) -> TableCell<'a> {
	TableCell::new(text, style)
}

impl Table {
	pub fn new<'data, I>(pdf: &PdfWriter, width: Length<Mm>, columns: usize, position: BoxPosition, data: I) -> Result<Self, String>
	where
		I: IntoIterator,
		I::Item: AsRef<TableCell<'data>>,
	{
		if columns == 0 {
			return Ok(Self {
				columns,
				cells: Vec::new(),
				size: Size2::new(0.0, 0.0),
				position,
			})
		}

		// Create initial cells without width restriction to get natural widths.
		let mut cells: Vec<_> = data
			.into_iter()
			.map(|c| {
				let c = c.as_ref();
				let position = BoxPosition::at(Point2::new(0.0, 0.0));
				pdf.text_box(c.text, c.style, position, None)
			})
			.collect::<Result<_, _>>()?;

		// Compute maximum natural widths of the columns.
		let mut natural_widths = vec![mm(0.0); columns];
		for (i, cell) in cells.iter().enumerate() {
			let column = i % columns;
			natural_widths[column] = natural_widths[column].max(cell.logical_width());
		}

		// Divide actual width according to natural width.
		let widths = divide_width(&natural_widths, width);
		for (i, cell) in cells.iter_mut().enumerate() {
			cell.set_width(Some(widths[i % columns]))
		}

		// Lay-out all cells in a table grid.
		let mut cursor: Point2<Mm> = Point2::new(0.0, 0.0);
		let mut row_height = mm(0.0);
		for (i, cell) in cells.iter_mut().enumerate() {
			if i % columns == 0 {
				cursor.x = 0.0;
				cursor.y += row_height.get();
				row_height = mm(0.0);
			}
			row_height = row_height.max(cell.logical_height());
			cell.set_position(BoxPosition::at(cursor));
			cursor.x += widths[i % columns].get();
		}

		cursor.y += row_height.get();
		let size = Size2::new(width.get(), cursor.y);

		let baseline = cells
			.get(0)
			.map(|text| text.baseline())
			.unwrap_or(mm(0.0));
		let offset = position.point.to_vector() + position.alignment_offset(size, baseline);
		for cell in &mut cells {
			cell.position.point += offset;
		}

		Ok(Self {
			columns,
			cells,
			size,
			position,
		})
	}

	pub fn draw(&self, page: &Page) {
		for cell in &self.cells {
			cell.draw(page);
		}
	}

	pub fn draw_horizontal_border<R: std::ops::RangeBounds<usize>>(&self, page: &Page, row: usize, columns: R, width: Length<Pt>) {
		let y = if row == self.rows() {
			mm(self.size.height)
		} else {
			mm(self.cells[row * self.columns].position().point.y)
		};

		let x1 = match columns.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};

		let x2 = match columns.end_bound() {
			std::ops::Bound::Included(&i) => i + 1,
			std::ops::Bound::Excluded(&i) => i,
			std::ops::Bound::Unbounded => self.columns - 1,
		};
		assert!(x1 < self.columns);
		assert!(x2 < self.columns);

		let x1 = self.get_column_start(x1);
		let x2 = self.get_column_start(x2 + 1);

		let y = y * PT_PER_MM;
		let x1 = x1 * PT_PER_MM;
		let x2 = x2 * PT_PER_MM;
		page.cairo.save();
		page.cairo.move_to(x1.get(), y.get());
		page.cairo.line_to(x2.get(), y.get());
		page.cairo.set_line_width(width.get());
		page.cairo.set_source(&cairo::SolidPattern::from_rgba(0.0, 0.0, 0.0, 1.0));
		page.cairo.stroke();
		page.cairo.restore();
	}

	fn rows(&self) -> usize {
		if self.cells.is_empty() {
			0
		} else {
			(self.cells.len() + self.columns - 1) / self.columns
		}
	}

	fn get_column_start(&self, index: usize) -> Length<Mm> {
		if index < self.columns {
			mm(self.cells[index].compute_extents().logical.min.x)
		} else if index == self.columns {
			self.get_column_start(0) + mm(self.size.width)
		} else {
			panic!("index out of bounds");
		}
	}
}

impl<'a> AsRef<TableCell<'a>> for &'_ TableCell<'a> {
	fn as_ref(&self) -> &TableCell<'a> {
		*self
	}
}

fn divide_width<U>(natural_widths: &[Length<U>], available_width: Length<U>) -> Vec<Length<U>> {
	let count = natural_widths.len();
	let total_natural = natural_widths
		.iter()
		.sum::<Length<U>>()
		.max(Length::new(1.0));
	let fair = available_width / count as f64;

	// If we have room to spare, just divide it evenly.
	if total_natural <= available_width {
		return vec![fair; count];
	}

	let mut dividable = Length::new(0.0); // How much space can we divide over shrunk columns?
	let mut total_shrunk = Length::new(0.0); // How much space do the shrunk columns want in total?
	for &natural in natural_widths {
		if natural <= fair {
			dividable += fair - natural;
		} else {
			dividable += fair;
			total_shrunk += natural;
		}
	}

	let mut widths = Vec::with_capacity(count);
	for &natural in natural_widths {
		if natural <= fair {
			widths.push(natural);
		} else {
			widths.push(dividable * (natural / total_shrunk))
		}
	}
	widths
}
