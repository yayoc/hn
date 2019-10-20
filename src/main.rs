#![feature(try_trait)]

extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;

mod hn;

fn write_alt_screen_msg<W: Write>(screen: &mut W) {
    write!(screen, "{}{}Welcome to the alternate screen.{}Press '1' to switch to the main screen or '2' to switch to the alternate screen.{}Press 'q' to exit (and switch back to the main screen).",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Goto(1, 3),
           termion::cursor::Goto(1, 4)).unwrap();
}

fn main() {
    match hn::get_top_stories(10) {
        Ok(s) => println!("{:#?}", s),
        Err(e) => println!("{:#?}", e),
    };
    /*
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    write_alt_screen_msg(&mut screen);

    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('1') => {
                write!(screen, "{}", ToMainScreen).unwrap();
            }
            Key::Char('2') => {
                write!(screen, "{}", ToAlternateScreen).unwrap();
                write_alt_screen_msg(&mut screen);
            }
            _ => {}
        }
        screen.flush().unwrap();
    }
    write!(screen, "{}", termion::cursor::Show).unwrap();
    */
}
