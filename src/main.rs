use fltk::{
    app::{self, Scheme},
    button::Button,
    enums::Color,
    frame::Frame,
    image::PngImage,
    prelude::*,
    window::Window,
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Space,
    ClearEverything,
    BackSpace,
    Previous,
    Divide,
    Seven,
    Eight,
    Nine,
    Multiply,
    Four,
    Five,
    Six,
    Minus,
    One,
    Two,
    Three,
    Plus,
    Zero,
    Dot,
    EqualTo,
    LeftBracket,
    RightBracket,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = (400, 450);
    app::set_font_size(26);
    app::set_background_color(72, 72, 108);
    app::set_scheme(Scheme::Plastic);
    // Everything else
    let ((app, icon, (width, height)), (mut wind, mut previous, mut frame)) = (
        (
            app::App::default().with_scheme(Scheme::Gleam),
            PngImage::from_data(include_bytes!("icon.png")).unwrap(),
            (screen_width / 4, (2 * screen_height) / (3 * 6)),
        ),
        (
            Window::new(100, 100, screen_width, screen_height, "Calculator"),
            vec!["0".to_string()],
            Frame::new(0, 0, screen_width, screen_height / 3, "0"),
        ),
    );

    // Buttons
    let (
        (mut clear_everything, mut backspace, mut previous_button, mut divide),
        (mut seven, mut eight, mut nine, mut multiply),
        (mut four, mut five, mut six, mut minus),
        (mut one, mut two, mut three, mut plus),
        (mut zero, mut space, mut dot, mut equal_to),
        (mut left_bracket, mut right_bracket),
    ) = (
        (
            Button::new(0 * width, 150 + 0 * height, width, height, "CE"),
            Button::new(1 * width, 150 + 0 * height, width, height, "C"),
            Button::new(2 * width, 150 + 0 * height, width, height, "PREV"),
            Button::new(3 * width, 150 + 0 * height, width, height, "/"),
        ),
        (
            Button::new(0 * width, 150 + 1 * height, width, height, "7"),
            Button::new(1 * width, 150 + 1 * height, width, height, "8"),
            Button::new(2 * width, 150 + 1 * height, width, height, "9"),
            Button::new(3 * width, 150 + 1 * height, width, height, "*"),
        ),
        (
            Button::new(0 * width, 150 + 2 * height, width, height, "4"),
            Button::new(1 * width, 150 + 2 * height, width, height, "5"),
            Button::new(2 * width, 150 + 2 * height, width, height, "6"),
            Button::new(3 * width, 150 + 2 * height, width, height, "-"),
        ),
        (
            Button::new(0 * width, 150 + 3 * height, width, height, "1"),
            Button::new(1 * width, 150 + 3 * height, width, height, "2"),
            Button::new(2 * width, 150 + 3 * height, width, height, "3"),
            Button::new(3 * width, 150 + 3 * height, width, height, "+"),
        ),
        (
            Button::new(0 * width, 150 + 4 * height, width, height, "0"),
            Button::new(2 * width, 150 + 4 * height, width, height, "SPACE"),
            Button::new(1 * width, 150 + 4 * height, width, height, "."),
            Button::new(3 * width, 150 + 4 * height, width, height, "="),
        ),
        (
            Button::new(0 * width, 150 + 5 * height, width * 2, height, "("),
            Button::new(2 * width, 150 + 5 * height, width * 2, height, ")"),
        ),
    );

    wind.set_icon(Some(icon));

    divide.set_color(Color::DarkCyan);
    multiply.set_color(Color::DarkCyan);
    minus.set_color(Color::DarkCyan);
    equal_to.set_color(Color::DarkCyan);
    plus.set_color(Color::DarkCyan);
    clear_everything.set_color(Color::DarkGreen);
    backspace.set_color(Color::DarkGreen);
    previous_button.set_color(Color::DarkGreen);
    dot.set_color(Color::DarkRed);
    space.set_color(Color::DarkRed);
    left_bracket.set_color(Color::DarkYellow);
    right_bracket.set_color(Color::DarkYellow);

    one.set_color(Color::DarkMagenta);
    two.set_color(Color::DarkMagenta);
    three.set_color(Color::DarkMagenta);
    four.set_color(Color::DarkMagenta);
    five.set_color(Color::DarkMagenta);
    six.set_color(Color::DarkMagenta);
    seven.set_color(Color::DarkMagenta);
    eight.set_color(Color::DarkMagenta);
    nine.set_color(Color::DarkMagenta);
    zero.set_color(Color::DarkMagenta);

    wind.end();
    wind.show();

    let (sender, r) = app::channel::<Message>();

    one.emit(sender, Message::One);
    two.emit(sender, Message::Two);
    three.emit(sender, Message::Three);
    zero.emit(sender, Message::Zero);
    dot.emit(sender, Message::Dot);
    equal_to.emit(sender, Message::EqualTo);
    four.emit(sender, Message::Four);
    five.emit(sender, Message::Five);
    six.emit(sender, Message::Six);
    minus.emit(sender, Message::Minus);
    plus.emit(sender, Message::Plus);
    clear_everything.emit(sender, Message::ClearEverything);
    backspace.emit(sender, Message::BackSpace);
    previous_button.emit(sender, Message::Previous);
    divide.emit(sender, Message::Divide);
    seven.emit(sender, Message::Seven);
    eight.emit(sender, Message::Eight);
    nine.emit(sender, Message::Nine);
    multiply.emit(sender, Message::Multiply);
    space.emit(sender, Message::Space);
    left_bracket.emit(sender, Message::LeftBracket);
    right_bracket.emit(sender, Message::RightBracket);

    while app.wait() {
        if let Some(msg) = r.recv() {
            let mut current_string = frame.label();
            match msg {
                Message::LeftBracket => current_string += "(",
                Message::RightBracket => current_string += ")",
                Message::Space => {
                    current_string += " ";
                }
                Message::BackSpace => {
                    let _ = current_string.pop();
                }
                Message::ClearEverything => {
                    current_string = "0".to_string();
                    previous = vec!["0".to_string()];
                }
                Message::Divide => current_string += " / ",
                Message::Dot => current_string += ".",
                Message::Eight => current_string += "8",
                Message::EqualTo => {
                    previous.push(current_string.clone());
                    let result = meval::eval_str(&current_string);
                    match result {
                        Ok(value) => {
                            current_string = value.to_string();
                        }
                        Err(e) => {
                            current_string = "0".to_string();
                            println!("Error: {}", e);
                        }
                    }
                }
                Message::Five => current_string += "5",
                Message::Four => current_string += "4",
                Message::Minus => current_string += "-",
                Message::Multiply => current_string += " * ",
                Message::Nine => current_string += "9",
                Message::One => current_string += "1",
                Message::Plus => current_string += "+",
                Message::Previous => {
                    if previous.len() == 1 {
                        previous.push("0".to_string());
                    }
                    current_string = previous.pop().unwrap();
                }
                Message::Seven => current_string += "7",
                Message::Six => current_string += "6",
                Message::Three => current_string += "3",
                Message::Two => current_string += "2",
                Message::Zero => current_string += "0",
            }
            frame.set_label(current_string.as_str());
        }
    }

    Ok(())
}
