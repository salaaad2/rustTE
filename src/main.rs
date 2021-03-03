extern crate termion;

pub mod key_actions;

use key_actions::print_char;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use std::fmt;

pub struct SPos {
    pub xpos: i32,
    pub ypos: i32,
}

impl fmt::Display for SPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.xpos, self.ypos)
    }
}

fn edit(stdin: std::io::Stdin, stdout: std::io::Stdout) {
    let mut out = stdout.into_raw_mode().unwrap();
    let mut pos = SPos{ypos : 1, xpos : 1};

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => print_char(c, &mut pos),
            Key::Alt(c) => print!("^{}", c),
            Key::Ctrl(c) => print!("*{}", c),
            Key::Esc => print!("ESC"),
            Key::Left => print!("←"),
            Key::Right => print!("→"),
            Key::Up => print!("↑"),
            Key::Down => print!("↓"),
            Key::Backspace => print!("×"),
            _ => {}
        }
        out.flush().unwrap();
    }
    write!(out, "{}", termion::cursor::Show).unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    write!(stdout,
           "{}{}Welcome to JUTE, a very nice text editor. press q to exit. {}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
            .unwrap();
    stdout.flush().unwrap();
    edit(stdin, stdout);
}
