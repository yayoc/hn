use crate::hn;

/// The app state.
pub struct App {
    /// Hacker News stories.
    pub stories: Vec<hn::Story>,
    /// Current index of the focused story.
    pub cur_index: usize,
    /// If waiting for API requests or not.
    pub is_loading: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            stories: Vec::new(),
            cur_index: 0,
            is_loading: false,
        }
    }
}

impl App {
    pub fn start_loading(&mut self) {
        self.is_loading = true;
    }

    pub fn loaded(&mut self, stories: Vec<hn::Story>) {
        self.cur_index = 0;
        self.stories = stories;
        self.is_loading = false;
    }

    pub fn open_browser(&self) {
        let s = &self.stories[self.cur_index];
        match &s.url {
            Some(u) => {
                webbrowser::open(u.as_str()).expect("Can't open your browser.");
            }
            None => {}
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cur_index > 0 {
            self.cur_index -= 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cur_index < self.stories.len() - 1 {
            self.cur_index += 1;
        }
    }

    pub fn cursor_jump_up(&mut self) {
        let jump_row = 10;
        match self.cur_index.checked_sub(jump_row) {
            Some(s) => self.cur_index = s,
            None => self.cur_index = 0,
        }
    }

    pub fn cursor_jump_down(&mut self) {
        let jump_row = 10;
        if self.cur_index < self.stories.len() - jump_row {
            self.cur_index += jump_row;
        } else {
            self.cur_index = if !self.stories.is_empty() {
                self.stories.len() - 1
            } else {
                0
            };
        }
    }

    pub fn cursor_jump_top(&mut self) {
        self.cur_index = 0;
    }

    pub fn cursor_jump_bottom(&mut self) {
        self.cur_index = if !self.stories.is_empty() {
            self.stories.len() - 1
        } else {
            0
        };
    }
}
