extern crate termion;

pub mod key_actions;

use key_actions::print_char;
use key_actions::key_left;
use key_actions::key_up;
use key_actions::back_space;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use std::io::{Write, stdout, stdin};
use std::fmt;
use std::env;
use std::fs::File;

pub struct Rute {
    pub spos: u16,
    pub x: u16,
    pub y: u16,
    pub size: (u16, u16),
    pub l: String,
}

impl fmt::Display for Rute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.size.0, self.size.1)
    }
}

fn edit(mut filename: std::fs::File, stdin: std::io::Stdin, stdout: std::io::Stdout) {
    let mut out = stdout.into_raw_mode().unwrap();
    let mut pos = Rute{spos : 1,
                       y : 1,
                       x : 2,
                       size: terminal_size().unwrap(),
                       l: "a".to_string()};

        for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => print_char(c, &mut pos),
            Key::Alt(c) => print!("^{}", c),
            Key::Ctrl(c) => print!("*{}", c),
            Key::Esc => print!("ESC"),
            Key::Left => key_left(&mut pos),
            Key::Right => print!("→"),
            Key::Up => key_up(&mut pos),
            Key::Down => print!("↓"),
            Key::Backspace => back_space(&mut pos),
            _ => {}
        }
        out.flush().unwrap();
    }
    write!(out, "{}", termion::cursor::Show).unwrap();
    if let Err(e) = writeln!(& mut filename, "{}", pos.l) {
        println!("Writei: eror: {}", e.to_string());
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();
    let filename;
    let args: Vec<String> = env::args().collect();
    let mut input = "default.rute";

    if args.len() == 0x2
    {
        input = &args[1];
    }
    filename = File::create(input).unwrap();
    write!(stdout,
           "{}{}Welcome to RUTE, a very nice text editor. press q to exit. {}{}{}",
           termion::clear::All,
           termion::cursor::Goto(10, 5),
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1),
           termion::cursor::Show)
        .unwrap();
    stdout.flush().unwrap();
    edit(filename, stdin, stdout);
}
