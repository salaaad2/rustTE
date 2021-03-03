use crate::SPos;

pub fn print_char(c: char, pos: &mut SPos) {
    print!("{}", c);
    pos.xpos = pos.xpos + 1;
    print!("{}", termion::cursor::Goto(pos.xpos, pos.ypos));
}
