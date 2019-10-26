#![feature(try_trait)]

extern crate termion;

use std::cmp::{max, min};
use std::io::{stdin, stdout, Write};
use std::path::Component::CurDir;
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::screen::*;
use webbrowser;

mod hn;

struct Cursor {
    row: usize,
    column: usize,
}

struct App {
    stories: Vec<hn::Story>,
    cursor: Cursor,
    row_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            stories: Vec::new(),
            cursor: Cursor { row: 0, column: 0 },
            row_offset: 0,
        }
    }
}

impl App {
    fn terminal_size() -> (usize, usize) {
        let (cols, rows) = termion::terminal_size().unwrap();
        (rows as usize, cols as usize)
    }

    fn open(&mut self, stories: Vec<hn::Story>) {
        self.stories = stories;
        self.cursor = Cursor { row: 0, column: 0 };
        self.row_offset = 0;
    }

    fn draw<T: Write>(&self, out: &mut T) {
        let (rows, cols) = Self::terminal_size();

        write!(out, "{}", clear::All);
        write!(out, "{}", cursor::Goto(1, 1));

        for i in self.row_offset..self.row_offset + rows {
            let s = self.stories.get(i);
            if s.is_none() {
                break;
            }

            for c in s.unwrap().title.chars() {
                write!(out, "{}", c);
            }
            if i < self.row_offset + rows - 1 {
                write!(out, "\r\n");
            }
        }
        let mut row =
            max(1, self.cursor.row as u16 + 1 - self.row_offset as u16);
        if self.cursor.row == self.stories.len() {
            row -= 1;
        }
        write!(
            out,
            "{}",
            cursor::Goto(
                self.cursor.column as u16 + 1,
                row
            )
        );
        // For debug.
        write!(out, "{}", self.row_offset);
        out.flush().unwrap();
    }

    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cursor.row);
        if self.cursor.row + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cursor.row + 1 - rows);
        }
        println!("{:#?}", self.row_offset);
    }

    fn open_browser(&self) {
        let s = &self.stories[self.cursor.row];
        match &s.url {
            Some(u) => {
                webbrowser::open(u.as_str());
            }
            None => {}
        }
    }

    fn cursor_up(&mut self) {
        if self.cursor.row > 0 {
            self.cursor.row -= 1;
        }
        self.scroll();
    }

    fn cursor_down(&mut self) {
        if self.cursor.row < self.stories.len() {
            self.cursor.row += 1;
        }
        self.scroll();
    }
}

fn main() {
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", clear::All);
    write!(stdout, "{}", "loading...");
    stdout.flush().unwrap();

    let mut stories: Vec<hn::Story> = Vec::new();
    match hn::get_top_stories(50) {
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
            }
            Event::Key(Key::Up) => {
                app.cursor_up();
            }
            Event::Key(Key::Char('k')) => {
                app.cursor_up();
            }
            Event::Key(Key::Down) => {
                app.cursor_down();
            }
            Event::Key(Key::Char('j')) => {
                app.cursor_down();
            }
            Event::Key(Key::Char('\n')) => {
                app.open_browser();
            }
            _ => {}
        }
        app.draw(&mut stdout);
    }
}
