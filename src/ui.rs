use crate::picker::Picker;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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
    if app.has_preview() {
        // Split screen horizontally for preview mode
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.area());

        // Left side - normal picker interface
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(main_chunks[0]);

        // update the height before rendering so this doesn't get out of sync
        app.update_height(left_chunks[2].height - 3);

        // render the sections of the display now that everything is setup and updated
        render_help(f, left_chunks[0], app);
        render_search_input(f, app, left_chunks[1]);
        render_items(f, app, left_chunks[2]);

        // Right side - preview
        render_preview(f, app, main_chunks[1]);
    } else {
        // Normal full-screen mode
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.area());

        // update the height before rendering so this doesn't get out of sync
        // TODO ensure that 3 is always correct or pull the correct value that takes terminal resizing into account
        app.update_height(chunks[2].height - 3);

        // render the sections of the display now that everything is setup and updated
        render_help(f, chunks[0], app);
        render_search_input(f, app, chunks[1]);
        render_items(f, app, chunks[2]);
    }
}

fn render_help<T>(f: &mut Frame, area: Rect, app: &Picker<T>)
where
    T: Sync + Send + Display,
{
    let left_text = vec![Line::from(vec![
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

    let right_text = vec![Line::from(vec![
        Span::styled(
            app.running_threads().to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" indexers"),
    ])
    .right_aligned()];

    let spans = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(12)])
        .split(area);

    let left_paragraph = Paragraph::new(left_text);
    let right_paragraph = Paragraph::new(right_text);
    f.render_widget(left_paragraph, spans[0]);
    f.render_widget(right_paragraph, spans[1]);
}

fn render_search_input<T>(f: &mut Frame, app: &Picker<T>, area: Rect)
where
    T: Sync + Send + Display,
{
    let (text, cursor_index, title) = match app.mode {
        crate::picker::PickerMode::Search => (&app.query, app.query_index, "Search"),
        crate::picker::PickerMode::Editing => (&app.editing_text, app.editing_index, "Editing"),
    };

    // Split the text at the cursor position
    let before_cursor = text.chars().take(cursor_index).collect::<String>();
    let cursor_char = text.chars().nth(cursor_index).unwrap_or(' ');
    let after_cursor = text.chars().skip(cursor_index + 1).collect::<String>();

    // Create a line with styled spans for before, cursor, and after
    let line = Line::from(vec![
        Span::raw(before_cursor),
        Span::styled(cursor_char.to_string(), Style::default().bg(Color::Blue)),
        Span::raw(after_cursor),
    ]);

    let input = Paragraph::new(line)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title(title));

    let snapshot = app.snapshot();
    let item_count_text = vec![Line::from(vec![Span::styled(
        format!(
            "{}/{}",
            snapshot.matched_item_count(),
            snapshot.item_count()
        ),
        Style::default().add_modifier(Modifier::BOLD),
    )])
    .right_aligned()];

    let item_count: Paragraph<'_> = Paragraph::new(item_count_text)
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title("Status"));

    let spans = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(17)])
        .split(area);

    f.render_widget(input, spans[0]);
    f.render_widget(item_count, spans[1]);
}

fn render_items<T>(f: &mut Frame, app: &mut Picker<T>, area: Rect)
where
    T: Sync + Send + Display,
{
    if app.matched_item_count() > 0 {
        let items: Vec<ListItem> = app
            .matched_items()
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
            &mut ratatui::widgets::ListState::default().with_selected(Some(
                app.current_index
                    // we need to correct the index here so that it's adjusted for the slice we're currently rendering
                    .saturating_sub(app.first_visible_item_index()) as usize,
            )),
        );
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // .margin(2)
            .constraints(
                [
                    Constraint::Min(1),
                    Constraint::Length(1),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.area());

        let no_items_paragraph = Paragraph::new("No items found").alignment(Alignment::Center);
        f.render_widget(no_items_paragraph, chunks[1]);
    }
}

fn render_preview<T>(f: &mut Frame, app: &Picker<T>, area: Rect)
where
    T: Sync + Send + Display,
{
    let preview_text = app.preview_output();
    let lines: Vec<Line> = preview_text
        .lines()
        .map(|line| Line::from(line.to_string()))
        .collect();

    let preview = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Preview"))
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(preview, area);
}
