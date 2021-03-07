use pdf_writer::{PdfWriter, TextStyle, FontSpec, TextAlign, A4, mm, pt};

fn main () {
	let file = std::io::BufWriter::new(std::fs::File::create("foo.pdf").unwrap());
	let margins = [mm(30.0), mm(20.0), mm(30.0), mm(20.0)];
	let mut writer = PdfWriter::new(file, A4).unwrap();

	let p1 = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
	let p2 = "In semper sapien quis ligula egestas auctor. Nulla eget imperdiet urna. Aliquam viverra, lacus nec egestas pharetra, nulla magna iaculis libero, et suscipit magna tortor et metus. Aliquam erat volutpat. Suspendisse pellentesque ante ut arcu pharetra ultricies. Ut finibus vel nulla ac hendrerit. Morbi non ligula a mauris vulputate pulvinar non ut tortor. Proin elit velit, mollis nec risus at, tincidunt sagittis elit. Maecenas vel faucibus massa, nec consectetur mauris. Sed elementum velit maximus, porta nulla eu, aliquam tellus. Quisque eleifend sem ipsum, ac bibendum turpis venenatis at. Nunc condimentum nibh sit amet eros congue, a ornare odio consequat.";
	let p3 = "Nam non massa ut tellus rutrum commodo a sed ex. Integer porttitor, velit sed fermentum rhoncus, ligula lectus euismod neque, vel efficitur arcu enim id lectus. Praesent rutrum ante a accumsan euismod. Sed erat turpis, consectetur eu enim in, maximus facilisis neque. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Praesent eu libero eget mi imperdiet ultricies sit amet at diam. Praesent vitae vulputate lorem. Nunc porttitor est quis ex porta, ac fermentum lorem fringilla. Aliquam pellentesque vel mi id egestas.";
	let p4 = "Nam lobortis mauris nunc, quis lobortis felis fermentum ac. Nunc vel dui placerat, mattis lectus eget, varius sem. Proin lacinia consectetur scelerisque. Maecenas viverra laoreet risus ut eleifend. Sed eget sollicitudin massa. Nullam vitae dolor faucibus, varius metus ut, laoreet arcu. Sed vitae neque rutrum, pharetra nulla non, viverra lorem. Duis egestas non metus sit amet sodales. Quisque vel vestibulum sapien, ac bibendum quam. Donec fermentum vestibulum arcu, vel ullamcorper massa lobortis quis. Nullam tristique feugiat enim non mollis. Aliquam quis vestibulum eros, eget congue mauris. Donec lobortis ac sapien et vulputate. Mauris luctus leo id sollicitudin tempor. Aenean pretium euismod scelerisque.";
	let p5 = "Nam est tortor, semper dignissim aliquet at, pulvinar sit amet libero. Fusce sodales, nisl sit amet dignissim consequat, arcu felis dictum diam, ac euismod metus enim vitae erat. Nullam sed urna placerat, sollicitudin quam a, dignissim eros. Donec aliquet gravida porttitor. Morbi et pharetra leo. Mauris elementum eu orci a eleifend. Sed at vestibulum nisi, nec accumsan odio.";

	let plain = TextStyle {
		font: FontSpec::plain("serif", pt(10.0)),
		align: TextAlign::Left,
		justify: true,
	};

	let heading = TextStyle {
		font: FontSpec::bold("serif", pt(16.0)),
		align: TextAlign::Left,
		justify: true,
	};

	let mut page = writer.page(margins);
	page.write_text("PDF writer using cairo/pango", &heading).unwrap();
	page.write_text("", &plain).unwrap();
	page.write_text(p1, &plain).unwrap();
	page.write_text("", &plain).unwrap();
	page.write_text(p2, &plain).unwrap();
	page.write_text("", &plain).unwrap();
	page.write_text(p3, &plain).unwrap();
	page.write_text("", &plain).unwrap();
	page.write_text(p4, &plain).unwrap();
	page.write_text("", &plain).unwrap();
	page.write_text(p5, &plain).unwrap();
	page.emit();
}
