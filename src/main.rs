use fltk::{
    app::{self, Scheme},
    frame::Frame,
    image::PngImage,
    prelude::*,
    window::Window,
};

fn main() {
    let app = app::App::default().with_scheme(Scheme::Gtk);
    let mut wind = Window::new(100, 100, 400, 450, "Calculator");

    let icon = PngImage::from_data(include_bytes!("icon.png")).unwrap();
    wind.set_icon(Some(icon));

    let mut frame = Frame::new(0, 0, 400, 150, "Yokuso!");
    wind.add(&frame);

    wind.end();
    wind.show();
    app.run().unwrap();
}
