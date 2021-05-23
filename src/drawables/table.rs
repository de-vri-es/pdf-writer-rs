use crate::{
	Drawable,
	Length,
	Margins,
	Surface,
	TextAlignment,
	TextBox,
	Vector2,
};

pub struct Table {
	columns: Vec<ColumnSpec>,
	cell_padding: Margins,
	cells: Vec<TableCell>,

	max_width: Option<Length>,
}

#[derive(Debug, Clone)]
pub struct ColumnSpec {
	pub grow: bool,
	pub max_width: Option<Length>,
}

struct TableCell {
	text: TextBox,
	alignment: TextAlignment,
	padding: Option<Margins>,
}

pub struct TableLayout {
	table: Table,
	column_widths: Vec<Length>,
	row_heights: Vec<Length>,
}

impl Default for Table {
	fn default() -> Self {
		Self::new()
	}
}

impl Table {
	pub fn new() -> Self {
		Self {
			cell_padding: Margins::vh(Length::from_pt(1.0), Length::from_pt(4.0)),
			columns: Vec::new(),
			cells: Vec::new(),
			max_width: None,
		}
	}

	/// Set the max width for the table.
	pub fn set_max_width(mut self, max_width: impl Into<Option<Length>>) -> Self {
		self.max_width = max_width.into();
		self
	}

	/// Set the cell padding.
	pub fn set_cell_padding(mut self, padding: Margins) -> Self {
		self.cell_padding = padding;
		self
	}

	/// Set the columns of the table.
	///
	/// This replaces all existing column specifications with the given ones.
	pub fn set_columns(mut self, columns: Vec<ColumnSpec>) -> Self {
		self.columns = columns;
		self
	}

	/// Add a column to the table.
	///
	/// This replaces all existing column specifications with the given ones.
	pub fn add_column(mut self, grow: bool, max_width: Option<Length>) -> Self {
		self.columns.push(ColumnSpec {
			grow,
			max_width,
		});
		self
	}

	/// Add a cell to the table.
	///
	/// Cells must be added in row major order.
	pub fn add_cell(mut self, text: TextBox, alignment: TextAlignment, padding: impl Into<Option<Margins>>) -> Self {
		let padding = padding.into();
		self.cells.push(TableCell { text, alignment, padding });
		self
	}

	pub fn rows(&self) -> usize {
		if self.cells.is_empty() {
			0
		} else {
			(self.cells.len() + self.columns.len() - 1) / self.columns.len()
		}
	}

	/// Compute the layout of the table.
	pub fn layout(mut self) -> TableLayout {
		if self.columns.is_empty() || self.cells.is_empty() {
			return TableLayout {
				table: self,
				column_widths: Vec::new(),
				row_heights: Vec::new(),
			};
		}

		let column_count = self.columns.len();

		// Compute maximum natural widths of the columns.
		let mut natural_widths = vec![Length::zero(); column_count];
		for (i, cell) in self.cells.iter().enumerate() {
			let column = i % column_count;
			let padding = cell.padding.as_ref().unwrap_or(&self.cell_padding);
			natural_widths[column] = natural_widths[column].max(cell.text.compute_natural_width() + padding.total_horizontal());
		}

		// Divide maximum width according to natural width.
		let column_widths;
		if let Some(max_width) = self.max_width {
			column_widths = divide_width(&self.columns, &natural_widths, max_width);
			for (i, cell) in self.cells.iter_mut().enumerate() {
				let column = i % column_count;
				let padding = cell.padding.as_ref().unwrap_or(&self.cell_padding);
				crate::DrawableMut::set_max_width(&mut cell.text, Some(column_widths[column] - padding.total_horizontal()));
			}
		} else {
			column_widths = natural_widths;
		};

		let mut row_heights = Vec::new();
		let mut current_row_height = Length::zero();
		for (i, cell) in self.cells.iter().enumerate() {
			let padding = cell.padding.as_ref().unwrap_or(&self.cell_padding);
			current_row_height = current_row_height.max(cell.text.compute_height() + padding.total_vertical());
			if (i + 1) % column_count == 0 {
				row_heights.push(current_row_height);
				current_row_height = Length::zero()
			}
		}

		TableLayout {
			table: self,
			column_widths,
			row_heights,
		}
	}
}

impl TableLayout {
	/// Destroy the table layout to get back the original table.
	pub fn into_table(self) -> Table {
		self.table
	}

	fn col_start(&self, index: usize) -> Length {
		self.column_widths[..index].iter().sum()
	}

	fn col_end(&self, index: usize) -> Length {
		self.col_start(index + 1)
	}

	fn row_start(&self, index: usize) -> Length {
		self.row_heights[..index].iter().sum()
	}

	pub fn draw(&self, surface: &Surface, position: Vector2) {
		let mut cursor = position;
		let column_count = self.table.columns.len();
		for (i, cell) in self.table.cells.iter().enumerate() {
			let padding = cell.padding.as_ref().unwrap_or(&self.table.cell_padding);
			let mut with_offset = crate::Offset::new(&cell.text, Vector2::new(padding.left, padding.top));
			match cell.alignment {
				TextAlignment::Left => (),
				TextAlignment::Right => with_offset.anchor_right().offset_mut().x += self.column_widths[i % column_count],
				TextAlignment::Center => with_offset.anchor_center_x().offset_mut().x += 0.5 * self.column_widths[i % column_count],
			};
			with_offset.draw(surface, cursor);
			if (i + 1) % column_count == 0 {
				cursor.x = position.x;
				cursor.y += self.row_heights[i / column_count];
			} else {
				cursor.x += self.column_widths[i % column_count];
			}
		}
	}

	/// Get the baseline of a table row.
	///
	/// The baseline is computed based on the first line of text in the first cell of the row.
	pub fn comput_baseline(&self, row: usize) -> Length {
		let cell = &self.table.cells[row * self.table.columns.len()];
		self.row_start(row) + cell.text.compute_baseline()
	}

	pub fn draw_horizontal_border<R: std::ops::RangeBounds<usize>>(&self, surface: &Surface, row: usize, columns: R, width: Length) {
		let y = self.row_start(row);

		let x1 = match columns.start_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i + 1,
			std::ops::Bound::Unbounded => 0,
		};

		let x2 = match columns.end_bound() {
			std::ops::Bound::Included(&i) => i,
			std::ops::Bound::Excluded(&i) => i - 1,
			std::ops::Bound::Unbounded => self.table.columns.len() - 1,
		};
		assert!(x1 < self.table.columns.len());
		assert!(x2 < self.table.columns.len());

		let x1 = self.col_start(x1);
		let x2 = self.col_end(x2);

		surface.cairo.save();
		surface.cairo.move_to(x1.as_pt(), y.as_pt());
		surface.cairo.line_to(x2.as_pt(), y.as_pt());
		surface.cairo.set_line_width(width.as_pt());
		surface.cairo.set_source(&cairo::SolidPattern::from_rgba(0.0, 0.0, 0.0, 1.0));
		surface.cairo.stroke();
		surface.cairo.restore();
	}

	pub fn compute_size(&self) -> Vector2 {
		Vector2 {
			x: self.col_start(self.table.columns.len()),
			y: self.row_start(self.table.rows()),
		}
	}

	pub fn compute_width(&self) -> Length {
		self.col_start(self.table.columns.len())
	}

	pub fn compute_height(&self) -> Length {
		self.row_start(self.table.rows())
	}
}

fn divide_width(columns: &[ColumnSpec], natural_widths: &[Length], available_width: Length) -> Vec<Length> {
	debug_assert!(columns.len() == natural_widths.len());

	let count = natural_widths.len();
	let total_natural = natural_widths.iter().sum::<Length>().max(Length::from_mm(1.0));
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

	let mut dividable = Length::zero(); // How much space can we divide over shrunk columns?
	let mut total_shrunk = Length::zero(); // How much space do the shrunk columns want in total?
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
