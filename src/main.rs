#![feature(try_trait)]

extern crate clap;
extern crate termion;

use clap::{App as ClapApp, Arg};
use std::cmp::{max, min};
use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use webbrowser;

mod hn;

struct App {
    stories: Vec<hn::Story>,
    cur_index: usize,
    row_offset: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            stories: Vec::new(),
            cur_index: 0,
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
        self.cur_index = 0;
        self.row_offset = 0;
    }

    fn draw<T: Write>(&self, out: &mut T) {
        let (rows, _cols) = Self::terminal_size();

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
        let cursor_row = max(1, self.cur_index as u16 + 1 - self.row_offset as u16);
        write!(out, "{}", cursor::Goto(1, cursor_row));
        out.flush().unwrap();
    }

    fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cur_index);
        if self.cur_index + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cur_index + 1 - rows);
        }
    }

    fn open_browser(&self) {
        let s = &self.stories[self.cur_index];
        match &s.url {
            Some(u) => {
                webbrowser::open(u.as_str());
            }
            None => {}
        }
    }

    fn cursor_up(&mut self) {
        if self.cur_index > 0 {
            self.cur_index -= 1;
        }
        self.scroll();
    }

    fn cursor_down(&mut self) {
        if self.cur_index < self.stories.len() - 1 {
            self.cur_index += 1;
        }
        self.scroll();
    }

    fn cursor_jump_up(&mut self) {
        let jump_row = 10;
        match self.cur_index.checked_sub(jump_row) {
            Some(s) => self.cur_index = s,
            None => self.cur_index = 0,
        }
        self.scroll();
    }

    fn cursor_jump_down(&mut self) {
        let jump_row = 10;
        if self.cur_index < self.stories.len() - jump_row {
            self.cur_index += jump_row;
        } else {
            self.cur_index = if self.stories.len() > 0 {
                self.stories.len() - 1
            } else {
                0
            };
        }
        self.scroll();
    }

    fn cursor_jump_top(&mut self) {
        self.cur_index = 0;
        self.scroll();
    }

    fn cursor_jump_bottom(&mut self) {
        self.cur_index = if self.stories.len() > 0 {
            self.stories.len() - 1
        } else {
            0
        };
        self.scroll();
    }
}

fn main() {
    let matches = ClapApp::new("hn")
        .version("0.0.1")
        .author("yayoc <hi@yayoc.com>")
        .about("CLI to browse Hacker News")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Sets a number of articles (defaults to 50)")
                .takes_value(true),
        )
        .get_matches();

    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    write!(stdout, "{}", clear::All);
    write!(stdout, "{}", "loading...");
    stdout.flush().unwrap();

    let mut stories: Vec<hn::Story> = Vec::new();
    let num = matches
        .value_of("number")
        .unwrap_or("50")
        .parse()
        .unwrap_or(50);
    match hn::get_top_stories(num) {
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
            Event::Key(Key::Ctrl('d')) => {
                app.cursor_jump_down();
            }
            Event::Key(Key::Ctrl('u')) => {
                app.cursor_jump_up();
            }
            Event::Key(Key::Char('g')) => {
                app.cursor_jump_top();
            },
            Event::Key(Key::Char('G')) => {
                app.cursor_jump_bottom();
            }
            _ => {}
        }
        app.draw(&mut stdout);
    }
}
