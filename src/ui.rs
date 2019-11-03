use std::io;

use crate::app;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &app::App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        if app.is_loading {
            draw_loading(&mut f, chunks[0]);
        } else {
            if app.stories.len() == 0 {
                draw_empty(&mut f, chunks[0]);
            } else {
                draw_list(&mut f, app, chunks[0]);
            }
        }
    })
}

fn draw_loading<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [Text::raw("Loading...")];
    Paragraph::new(text.iter()).wrap(true).render(f, area);
}

fn draw_empty<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [Text::raw(
        "Oops, No article. Please try to reload with CTRL+r",
    )];
    Paragraph::new(text.iter()).wrap(true).render(f, area);
}

fn draw_list<B>(f: &mut Frame<B>, app: &app::App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80)].as_ref())
        .split(area);
    let style = Style::default().fg(Color::White);
    let items: Vec<String> = app.stories.iter().map(|s| s.title_label()).collect();

    SelectableList::default()
        .block(Block::default().borders(Borders::ALL).title("HN Top Stories"))
        .items(&items)
        .select(Option::from(app.cur_index))
        .style(style)
        .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .render(f, chunks[0]);
}
