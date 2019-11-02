use std::io;

use crate::app;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{
    Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Marker, Paragraph, Row,
    SelectableList, Sparkline, Table, Tabs, Text, Widget,
};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &app::App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());
        draw_list(&mut f, app, chunks[0]);
    })
}

pub fn draw_list<B>(f: &mut Frame<B>, app: &app::App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80)].as_ref())
        .split(area);
    let style = Style::default().fg(Color::Black);

    let mut titles: Vec<&str> = Vec::new();
    for s in app.stories.iter() {
        titles.push(s.title.as_str());
    }
    SelectableList::default()
        .block(Block::default().borders(Borders::ALL).title("Top Stories"))
        .items(&titles)
        .select(Option::from(app.cur_index))
        .style(style)
        .highlight_style(style.fg(Color::LightGreen).modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .render(f, chunks[0]);
}

