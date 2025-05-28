use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::picker::Picker;

pub fn ui<T: std::marker::Sync + std::marker::Send + std::fmt::Display>(
    f: &mut Frame,
    app: &mut Picker<T>,
) {
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

fn render_search_input<T: std::marker::Sync + std::marker::Send>(
    f: &mut Frame,
    app: &Picker<T>,
    area: Rect,
) {
    let input = Paragraph::new(app.query.as_str())
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Search"));
    f.render_widget(input, area);
}

fn render_items<T: std::marker::Sync + std::marker::Send + std::fmt::Display>(
    f: &mut Frame,
    app: &Picker<T>,
    area: Rect,
) {
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
