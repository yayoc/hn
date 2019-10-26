#![feature(try_trait)]

extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::*;
use termion::screen::AlternateScreen;
use std::path::Component::CurDir;
use webbrowser;

mod hn;

struct Cursor {
    row: usize,
    column: usize,
}

struct App {
//    buffer: Vec<Vec<char>>,
    stories: Vec<hn::Story>,
    cursor: Cursor,
}

impl Default for App {
    fn default() -> Self {
        Self {
//            buffer: vec![Vec::new()],
            stories: Vec::new(),
            cursor: Cursor { row: 0, column: 0 },
        }
    }
}

impl App {
    fn open(&mut self, stories: Vec<hn::Story>) {
        self.stories = stories;
        self.cursor = Cursor { row: 0, column: 0 };
    }

    fn draw<T: Write>(&self, out: &mut T) {
        write!(out, "{}", clear::All);
        write!(out, "{}", cursor::Goto(1, 1));
        for s in &self.stories {
            for c in s.title.chars() {
                write!(out, "{}", c);
            }
            write!(out, "\r\n");
        }
        write!(
            out,
            "{}",
            cursor::Goto(self.cursor.column as u16 + 1, self.cursor.row as u16 + 1)
        );
        out.flush().unwrap();
    }

    fn open_browser(&self) {
        let s = &self.stories[self.cursor.row];
        match &s.url {
            Some(u) => {
                webbrowser::open(u.as_str());
            },
            None => {}
        }
    }

    fn cursor_up(&mut self) {
        if self.cursor.row > 0{
            self.cursor.row -= 1;
        }
    }

    fn cursor_down(&mut self) {
        if self.cursor.row < self.stories.len() {
            self.cursor.row += 1;
        }
    }
}


fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", clear::All);
    write!(stdout, "{}", "loading...");
    stdout.flush().unwrap();

    let mut stories: Vec<hn::Story> = Vec::new();
    match hn::get_top_stories(10) {
        Ok(mut s) => stories.append(&mut s),
        Err(e) => println!("{:#?}", e),
    };
    let mut app = App::default();
    app.open(stories);

    app.draw(&mut stdout);

    for evt in stdin.events() {
        match evt.unwrap() {
            Event::Key(Key::Ctrl('c')) => {
                return;
            },
            Event::Key(Key::Up) => {
                app.cursor_up();
            },
            Event::Key(Key::Char('k')) => {
                app.cursor_up();
            },
            Event::Key(Key::Down) => {
                app.cursor_down();
            },
            Event::Key(Key::Char('j')) => {
                app.cursor_down();
            },
            Event::Key(Key::Char('\n')) => {
                app.open_browser();
            },
            _ => {}
        }
        app.draw(&mut stdout);
    }
}
