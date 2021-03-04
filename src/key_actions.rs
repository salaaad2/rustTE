use termion::terminal_size;

use crate::SPos;

pub fn print_char(c: char, pos: &mut SPos) {
    let tuple = terminal_size().unwrap();
    print!("{}", c);
    pos.xpos = pos.xpos + 1;
    //println!("Size is {}", tuple.0);
    if pos.xpos == tuple.0
    {
        pos.ypos = pos.ypos + 1;
        pos.xpos = 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
    if c == '\n'
    {
        pos.ypos = pos.ypos + 1;
        pos.xpos = 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}
