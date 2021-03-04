use crate::SPos;

pub fn print_char(c: char, pos: &mut SPos) {
    print!("{}", c);
    pos.xpos = pos.xpos + 1;
    if pos.xpos == pos.size.0
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

pub fn back_space(pos: &mut SPos) {
    if pos.xpos > 0 && pos.ypos > 0
    {
        pos.xpos = pos.xpos - 1;
        print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
    }
}
