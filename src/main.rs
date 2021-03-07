extern crate termion;

pub mod key_actions;

use key_actions::print_char;
use key_actions::back_space;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use std::io::{Write, stdout, stdin};
use std::fmt;

pub struct SPos {
    pub xpos: u16,
    pub ypos: u16,
    pub size: (u16, u16),
    pub l: String,
}

impl fmt::Display for SPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.xpos, self.ypos, self.size.0, self.size.1)
    }
}

fn edit(stdin: std::io::Stdin, stdout: std::io::Stdout) {
    let mut out = stdout.into_raw_mode().unwrap();
    let mut pos = SPos{ypos : 1,
                       xpos : 1,
                       size: terminal_size().unwrap(),
                       l: "a".to_string()};

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
            Key::Backspace => back_space(&mut pos),
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
           "{}{}Welcome to RUTE, a very nice text editor. press q to exit. {}{}{}",
           termion::clear::All,
           termion::cursor::Goto(10, 5),
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1),
           termion::cursor::Show)
        .unwrap();
    stdout.flush().unwrap();
    edit(stdin, stdout);
}
