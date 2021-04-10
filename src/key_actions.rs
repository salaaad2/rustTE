use crate::Rute;

use std::convert::TryInto;

pub fn print_char(c: char, rute: &mut Rute) {
    if rute.x == rute.size.0
    {
        rute.y = rute.y + 1;
        rute.x = 0;
        print!("{}", termion::cursor::Goto(rute.x, rute.y));
    }
    if c == '\n'
    {
        rute.x = 0x0;
        rute.y = rute.y + 0x1;
        rute.l.insert(rute.spos.try_into().unwrap(), '\r');
        rute.spos = rute.spos + 1;
    }
    rute.l.insert(rute.spos.try_into().unwrap(), c);
    print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
    print!("{}", rute.l);
    rute.spos = rute.spos + 1;
    rute.x = rute.x + 1;
    print!("{}", termion::cursor::Goto(rute.x, rute.y));
}

pub fn key_up(rute: &mut Rute) {
    if rute.y >= 1
    {
        rute.x = 0x0;
        rute.y = rute.y - 0x1;
        rute.spos = rute.spos - rute.size.0; /* cool ternary operator incoming*/
        print!("{}", termion::cursor::Goto(rute.x, rute.y));
    }
}

pub fn key_down(rute: &mut Rute) {
    rute.x = 0x0;
    rute.y = rute.y + 0x1;
    rute.spos = rute.spos - rute.size.0; /* cool ternary operator incoming lol*/
    print!("{}", termion::cursor::Goto(rute.x, rute.y));
}

pub fn key_left(rute: &mut Rute) {
    if rute.x >= 1
    {
        rute.x = rute.x - 0x1;
        rute.spos = rute.spos - 0x1;
        print!("{}", termion::cursor::Goto(rute.x, rute.y));
    }
}

pub fn key_right(rute: &mut Rute) {
    if rute.x <= rute.spos
    {
        rute.x = rute.x + 0x1;
        rute.spos = rute.spos + 0x1;
        print!("{}", termion::cursor::Goto(rute.x, rute.y));
    }
}

pub fn back_space(rute: &mut Rute) {
    if rute.x > 0 && rute.y > 0 && rute.spos > 0
    {
        rute.x = rute.x - 0x1;
        rute.spos = rute.spos - 0x1;
        rute.l.remove(rute.spos.try_into().unwrap());
        print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::All);
        print!("{}", rute.l);
        print!("{}", termion::cursor::Goto(rute.x, rute.y));
    }
}
