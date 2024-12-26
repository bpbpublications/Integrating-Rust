use crossterm::style::{Color, Attribute, Stylize};

fn main() {

    let styled_hello = "Hello "
        .with(Color::Red)
        .on(Color::Cyan)
        .attribute(Attribute::Bold);
    
    let styled_rustacean = "Rustacean"
        .with(Color::Green)
        .on(Color::Red)
        .attribute(Attribute::Bold);

    println!("{}{}", styled_hello, styled_rustacean);
}
