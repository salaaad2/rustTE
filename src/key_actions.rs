use crate::SPos;

//use std::convert::TryInto;

pub fn print_char(c: char, pos: &mut SPos) {
    if pos.xpos == pos.size.0
    {
        pos.ypos = pos.ypos + 1;
        pos.xpos = 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
    if c == '\n'
    {
        pos.xpos = 1;
    }
    pos.l.push(c);
    print!("{}", termion::cursor::Goto(1, pos.xpos));
    print!("{}", pos.l);
    pos.xpos = pos.xpos + 1;
}

pub fn back_space(pos: &mut SPos) {
    if pos.xpos > 0 && pos.ypos > 0
    {
        pos.xpos = pos.xpos - 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}
