use crate::SPos;

use std::convert::TryInto;

pub fn print_char(c: char, pos: &mut SPos) {
    if pos.xpos == pos.size.0
    {
        pos.ypos = pos.ypos + 1;
        pos.xpos = 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
    if c == '\n'
    {
        pos.xpos = 0x0;
        pos.ypos = pos.ypos + 0x1;
        pos.l.insert(pos.spos.try_into().unwrap(), '\r');
        pos.spos = pos.spos + 1;
    }
    pos.l.insert(pos.spos.try_into().unwrap(), c);
    print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    print!("{}", pos.l);
    pos.spos = pos.spos + 1;
    pos.xpos = pos.xpos + 1;
    print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
}

pub fn key_up(pos: &mut SPos) {
    if pos.ypos >= 1
    {
        pos.xpos = 0x0;
        pos.ypos = pos.ypos - 0x1;
        pos.spos = pos.spos - pos.size.0; /* cool ternary operator incoming*/
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}

pub fn key_left(pos: &mut SPos) {
    if pos.xpos >= 1
    {
        pos.xpos = pos.xpos - 0x1;
        pos.spos = pos.spos - 0x1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}

pub fn back_space(pos: &mut SPos) {
    if pos.xpos > 0 && pos.ypos > 0
    {
        pos.xpos = pos.xpos - 0x1;
        pos.spos = pos.spos - 0x1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}
