extern crate termion;

use termion::{color, style};

use std::io;

fn main() {
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Bold, style::Reset);
    println!("{}Just plain italic", style::Italic);
    print!("{}{}Stuff", termion::clear::All, termion::cursor::Goto(40, 2))
}