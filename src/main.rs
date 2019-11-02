#![feature(try_trait)]

extern crate clap;
extern crate termion;

use clap::{App as ClapApp, Arg};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use termion::clear;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

mod app;
mod hn;
mod ui;

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
    stdout.flush().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor().unwrap();

    let mut a = app::App::default();
    a.start_loading();
    ui::draw(&mut terminal, &a).unwrap();

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
    a.loaded(stories);

    let (tx, rx) = channel();

    thread::spawn(move || {
        for c in stdin.events() {
            if let Ok(evt) = c {
                tx.send(evt).unwrap();
            }
        }
    });

    loop {
        ui::draw(&mut terminal, &a).unwrap();
        if let Ok(evt) = rx.recv_timeout(Duration::from_millis(16)) {
            match evt {
                Event::Key(Key::Ctrl('c')) => {
                    return;
                }
                Event::Key(Key::Up) => {
                    a.cursor_up();
                }
                Event::Key(Key::Char('k')) => {
                    a.cursor_up();
                }
                Event::Key(Key::Down) => {
                    a.cursor_down();
                }
                Event::Key(Key::Char('j')) => {
                    a.cursor_down();
                }
                Event::Key(Key::Char('\n')) => {
                    a.open_browser();
                }
                Event::Key(Key::Ctrl('d')) => {
                    a.cursor_jump_down();
                }
                Event::Key(Key::Ctrl('u')) => {
                    a.cursor_jump_up();
                }
                Event::Key(Key::Char('g')) => {
                    a.cursor_jump_top();
                }
                Event::Key(Key::Char('G')) => {
                    a.cursor_jump_bottom();
                }
                Event::Key(Key::Ctrl('r')) => {
                    a.start_loading();
                    ui::draw(&mut terminal, &a).unwrap();
                    let mut stories: Vec<hn::Story> = Vec::new();
                    match hn::get_top_stories(num) {
                        Ok(mut s) => stories.append(&mut s),
                        Err(e) => println!("{:#?}", e),
                    };
                    a.loaded(stories);
                }
                _ => {}
            }
        }
    }
}
