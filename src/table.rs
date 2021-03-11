use crate::{
	BoxPosition,
	Length,
	Mm,
	Page,
	PdfWriter,
	Point2,
	PT_PER_MM,
	Size2,
	TextBox,
	TextStyle,
	mm,
};

pub struct Table {
	cells: Vec<TextBox>,
	size: Size2<Mm>,
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
	pub fn new<'data, I>(pdf: &PdfWriter, width: Length<Mm>, columns: usize, data: I) -> Result<Self, String>
	where
		I: IntoIterator,
		I::Item: AsRef<TableCell<'data>>,
	{
		if columns == 0 {
			return Ok(Self {
				cells: Vec::new(),
				size: Size2::new(0.0, 0.0),
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

		let total_natural_width = natural_widths
			.iter()
			.sum::<Length<Mm>>()
			.max(Length::new(1.0));

		// Divide actual width according to natural width.
		let mut widths = Vec::with_capacity(columns);
		for i in 0..columns {
			let ratio = natural_widths[i] / total_natural_width;
			widths.push(width * ratio);
		}

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

		Ok(Self {
			cells,
			size: Size2::new(width.get(), cursor.y),
		})
	}

	pub fn draw(&self, page: &Page, position: &BoxPosition) {
		let baseline = self.cells
			.get(0)
			.map(|text| text.baseline())
			.unwrap_or(mm(0.0));
		let offset = position.point + position.alignment_offset(self.size, baseline);
		let offset = offset * PT_PER_MM;
		page.cairo.save();
		page.cairo.translate(offset.x, offset.y);
		for cell in &self.cells {
			cell.draw(page);
		}
		page.cairo.restore();
	}
}

impl<'a> AsRef<TableCell<'a>> for &'_ TableCell<'a> {
	fn as_ref(&self) -> &TableCell<'a> {
		*self
	}
}
