use crate::{
	BoxPosition,
	Length,
	Margins,
	Mm,
	MM_PER_PT,
	Page,
	PdfWriter,
	Point2,
	Pt,
	PT_PER_MM,
	Size2,
	TextAlign,
	TextBox,
	TextStyle,
	Vector2,
	mm,
	pt,
};

pub struct Table {
	position: BoxPosition,
	cell_padding: Margins<Mm>,
	columns: Vec<ColumnSpec>,
	cells: Vec<TableCell>,

	size: Size2<Mm>,
	column_widths: Vec<Length<Mm>>,
}

struct TableCell {
	text: TextBox,
	alignment: TextAlign,
}

pub struct TableBuilder<'a> {
	pdf_writer: &'a PdfWriter,
	max_width: Length<Mm>,
	position: BoxPosition,
	cell_padding: Margins<Mm>,
	columns: Vec<ColumnSpec>,
	cells: Vec<TableCell>,
}

impl<'a> TableBuilder<'a> {
	pub fn new(pdf_writer: &'a PdfWriter, max_width: Length<Mm>) -> Self {
		Self {
			pdf_writer,
			max_width,
			position: BoxPosition::at_xy(mm(0.0), mm(0.0)),
			cell_padding: Margins::vh(pt(1.0) * MM_PER_PT, pt(4.0) * MM_PER_PT),
			columns: Vec::new(),
			cells: Vec::new(),
		}
	}

	/// Set the max width for the table.
	pub fn max_width(&mut self, max_width: Length<Mm>) -> &mut Self {
		self.max_width = max_width;
		self
	}

	/// Set the table position.
	pub fn position(&mut self, position: BoxPosition) -> &mut Self {
		self.position = position;
		self
	}

	/// Set the cell padding.
	pub fn cell_padding(&mut self, padding: Margins<Mm>) -> &mut Self {
		self.cell_padding = padding;
		self
	}

	/// Set the columns of the table.
	///
	/// This replaces all existing column specifications with the given ones.
	pub fn set_columns(&mut self, columns: Vec<ColumnSpec>) -> &mut Self {
		self.columns = columns;
		self
	}

	/// Add a column to the table.
	///
	/// This replaces all existing column specifications with the given ones.
	pub fn add_column(&mut self, grow: bool, max_width: Option<Length<Mm>>) -> &mut Self {
		self.columns.push(ColumnSpec {
			grow,
			max_width,
		});
		self
	}

	/// Add a cell to the table.
	///
	/// Cells must be added in row major order.
	pub fn add_cell(&mut self, text: &str, style: &TextStyle) -> Result<&mut Self, String> {
		let alignment = style.align;
		let text = self.pdf_writer.text_box(text, style, BoxPosition::at_xy(mm(0.0), mm(0.0)), None)?;
		self.cells.push(TableCell { text, alignment });
		Ok(self)
	}

	pub fn build(self) -> Table {
		Table::new(self)
	}
}

#[derive(Debug, Clone)]
pub struct ColumnSpec {
	pub grow: bool,
	pub max_width: Option<Length<Mm>>,
}

impl Table {
	pub fn new(builder: TableBuilder) -> Table {
		let TableBuilder {
			max_width,
			position,
			columns,
			mut cells,
			cell_padding,
			..
		} = builder;

		if columns.is_empty() || cells.is_empty() {
			return Self {
				position,
				cell_padding,
				columns: Vec::new(),
				cells: Vec::new(),
				column_widths: Vec::new(),
				size: Size2::new(0.0, 0.0),
			}
		}

		let column_count = columns.len();

		// Compute maximum natural widths of the columns.
		let mut natural_widths = vec![mm(0.0); column_count];
		for (i, cell) in cells.iter().enumerate() {
			let column = i % column_count;
			natural_widths[column] = natural_widths[column].max(cell.text.logical_width() + cell_padding.total_horizontal());
		}

		// Divide maximum width according to natural width.
		let column_widths = divide_width(&columns, &natural_widths, max_width);

		// Calculate the start of the text within each column.
		// TODO: Deal with text alignment.
		let mut column_inner_start = Vec::with_capacity(column_count);
		let mut total_width = mm(0.0);
		for (&width, &natural) in column_widths.iter().zip(&natural_widths) {
			total_width += width;
			if natural < width {
				column_inner_start.push((width - natural) * 0.5 + cell_padding.left);
			} else {
				column_inner_start.push(cell_padding.left);
			}
		}

		// Lay-out all cells in a table grid.
		let mut cursor: Point2<Mm> = Point2::new(0.0, 0.0);
		let mut row_height = mm(0.0);
		for (i, cell) in cells.iter_mut().enumerate() {
			let column = i % column_count;
			if column == 0 {
				cursor.x = 0.0;
				cursor.y += (row_height + cell_padding.total_vertical()).get();
				row_height = mm(0.0);
			}

			let width = column_widths[column];
			cell.text.set_width(Some(width));

			let inner_offset = Vector2::new(column_inner_start[column].get(), cell_padding.top.get());
			row_height = row_height.max(cell.text.logical_height());
			let text_position = match cell.alignment {
				TextAlign::Left => BoxPosition::at(cursor + inner_offset),
				TextAlign::Center => BoxPosition::at(cursor + inner_offset + Vector2::new(width.get(), 0.0) * 0.5).anchor_hcenter(),
				TextAlign::Right => BoxPosition::at(cursor + inner_offset + Vector2::new(width.get(), 0.0)).anchor_right(),
			};
			cell.text.set_position(text_position);
			cursor.x += column_widths[column].get();
		}

		cursor.y += row_height.get();
		let size = Size2::new(total_width.get(), cursor.y);

		let baseline = cells
			.get(0)
			.map(|cell| cell.text.baseline())
			.unwrap_or(mm(0.0));
		let offset = position.point.to_vector() + position.alignment_offset(size, baseline);
		for cell in &mut cells {
			cell.text.position.point += offset;
		}

		Self {
			position,
			cell_padding,
			columns,
			cells,
			column_widths,
			size,
		}
	}

	pub fn draw(&self, page: &Page) {
		for cell in &self.cells {
			cell.text.draw(page);
		}
	}

	pub fn draw_horizontal_border<R: std::ops::RangeBounds<usize>>(&self, page: &Page, row: usize, columns: R, width: Length<Pt>) {
		let y = if row == self.rows() {
			mm(self.size.height)
		} else {
			mm(self.cells[row * self.columns.len()].text.position().point.y) - self.cell_padding.top
		};

		let x1 = match columns.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};

		let x2 = match columns.end_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i - 1,
			std::ops::Bound::Unbounded => self.columns.len() - 1,
		};
		assert!(x1 < self.columns.len());
		assert!(x2 < self.columns.len());

		let x1 = self.get_column_start(x1);
		let x2 = self.get_column_end(x2);

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
			(self.cells.len() + self.columns.len() - 1) / self.columns.len()
		}
	}

	fn get_column_start(&self, index: usize) -> Length<Mm> {
		assert!(index < self.columns.len());
		let offset = self.position.point.to_vector() + self.position.alignment_offset(self.size, mm(0.0));
		let width = self.column_widths[..index].iter().sum::<Length<Mm>>();
		mm(offset.x) + width
	}

	fn get_column_end(&self, index: usize) -> Length<Mm> {
		self.get_column_start(index) + self.column_widths[index]
	}
}

fn divide_width<U>(columns: &[ColumnSpec], natural_widths: &[Length<U>], available_width: Length<U>) -> Vec<Length<U>> {
	debug_assert!(columns.len() == natural_widths.len());

	let count = natural_widths.len();
	let total_natural = natural_widths
		.iter()
		.sum::<Length<U>>()
		.max(Length::new(1.0));
	let fair = available_width / count as f64;

	// If we have room to spare, just divide it evenly over columns that want to grow.
	if total_natural <= available_width {
		let excess = available_width - total_natural;
		let growers = columns.iter().filter(|x| x.grow).count();
		let spacing = excess / growers as f64;
		let mut widths = Vec::with_capacity(columns.len());
		for (spec, &natural) in columns.iter().zip(natural_widths) {
			if spec.grow {
				widths.push(natural + spacing);
			} else {
				widths.push(natural);
			}
		}
		return widths;
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
