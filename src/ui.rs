use crate::picker::Picker;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::fmt::Display;

pub fn ui<T>(f: &mut Frame, app: &mut Picker<T>)
where
    T: Sync + Send + Display,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.area());

    render_help(f, chunks[0]);
    render_search_input(f, app, chunks[1]);
    render_items(f, app, chunks[2]);

    // TODO ensure that 3 is always correct or pull the correct value that takes terminal resizing into account
    app.update_height(chunks[2].height - 3);
}

fn render_help(f: &mut Frame, area: Rect) {
    let text = vec![Line::from(vec![
        Span::raw("Press "),
        Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to navigate, "),
        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to select, "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to confirm, "),
        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit"),
    ])];

    let paragraph = Paragraph::new(text);
    f.render_widget(paragraph, area);
}

fn render_search_input<T>(f: &mut Frame, app: &Picker<T>, area: Rect)
where
    T: Sync + Send,
{
    // Split the query at the cursor position
    let before_cursor = app.query.chars().take(app.query_index).collect::<String>();
    let cursor_char = app.query.chars().nth(app.query_index).unwrap_or(' ');
    let after_cursor = app
        .query
        .chars()
        .skip(app.query_index + 1)
        .collect::<String>();

    // Create a line with styled spans for before, cursor, and after
    let line = Line::from(vec![
        Span::raw(before_cursor),
        Span::styled(cursor_char.to_string(), Style::default().bg(Color::Blue)),
        Span::raw(after_cursor),
    ]);

    let input = Paragraph::new(line)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, area);
}

fn render_items<T>(f: &mut Frame, app: &Picker<T>, area: Rect)
where
    T: Sync + Send + Display,
{
    let items: Vec<ListItem> = app
        .items()
        .iter()
        .map(|item| {
            let is_selected = item.is_selected();
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if is_selected { "✓ " } else { "  " };
            let content = format!("{}{}", prefix, item);

            ListItem::new(content).style(style)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Items"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(
        items,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.current_index as usize)),
    );
}
