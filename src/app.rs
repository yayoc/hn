use crate::hn;
use std::cmp::{max, min};
use std::io::Write;
use termion::clear;
use termion::cursor;

pub(crate) struct App {
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

    pub fn open(&mut self, stories: Vec<hn::Story>) {
        self.stories = stories;
        self.cur_index = 0;
        self.row_offset = 0;
    }

    pub fn draw<T: Write>(&self, out: &mut T) {
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

    pub fn scroll(&mut self) {
        let (rows, _) = Self::terminal_size();
        self.row_offset = min(self.row_offset, self.cur_index);
        if self.cur_index + 1 >= rows {
            self.row_offset = max(self.row_offset, self.cur_index + 1 - rows);
        }
    }

    pub fn open_browser(&self) {
        let s = &self.stories[self.cur_index];
        match &s.url {
            Some(u) => {
                webbrowser::open(u.as_str());
            }
            None => {}
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cur_index > 0 {
            self.cur_index -= 1;
        }
        self.scroll();
    }

    pub fn cursor_down(&mut self) {
        if self.cur_index < self.stories.len() - 1 {
            self.cur_index += 1;
        }
        self.scroll();
    }

    pub fn cursor_jump_up(&mut self) {
        let jump_row = 10;
        match self.cur_index.checked_sub(jump_row) {
            Some(s) => self.cur_index = s,
            None => self.cur_index = 0,
        }
        self.scroll();
    }

    pub fn cursor_jump_down(&mut self) {
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

    pub fn cursor_jump_top(&mut self) {
        self.cur_index = 0;
        self.scroll();
    }

    pub fn cursor_jump_bottom(&mut self) {
        self.cur_index = if self.stories.len() > 0 {
            self.stories.len() - 1
        } else {
            0
        };
        self.scroll();
    }
}
