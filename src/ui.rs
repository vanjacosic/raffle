use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(3)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            Line::from(Span::styled(
                t,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::UNDERLINED),
            ))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu")
                .border_type(BorderType::Rounded),
        )
        .select(app.tabs.active)
        .style(Style::default().fg(Color::LightBlue))
        .highlight_style(
            Style::default()
                .remove_modifier(Modifier::UNDERLINED)
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightRed)
                .bg(Color::Black),
        );

    frame.render_widget(tabs, chunks[0]);

    match app.tabs.active {
        0 => render_tab_1(app, frame, chunks[1]),
        1 => render_tab_2(app, frame, chunks[1]),
        2 => render_tab_3(app, frame, chunks[1]),
        _ => (),
    };
}

pub fn render_tab_1<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let text = Text::from(vec![
        Line::from(Span::styled(
            "R.A.F.F.L.E.",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "( Rapidly Assembled Faulty Fortune Locator Engine )",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(Span::raw("Made")),
        Line::from(Span::raw("for the")),
        Line::from(Span::raw("Copenhagen Rust Community 🦀🧡")),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::raw(
            "Press `Esc`, `Ctrl-C` or `q` to stop running.",
        )]),
    ]);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .padding(Padding {
                        left: 2,
                        right: 2,
                        top: 2,
                        bottom: 2,
                    }),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center),
        area,
    );
}

pub fn render_tab_2<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(area);

    let items: Vec<_> = app
        .list
        .items
        .iter()
        .map(|p| ListItem::new(Span::styled(p.name.clone(), Style::default())))
        .collect();

    let item_list = List::new(items)
        .block(
            Block::default()
                .title("All participants")
                .borders(Borders::ALL)
                .padding(Padding {
                    left: 2,
                    right: 2,
                    top: 1,
                    bottom: 1,
                }),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("-> ");

    frame.render_stateful_widget(item_list, chunks[0], &mut app.list.state);

    let mut text = Text::from(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{} ", app.list.items.len()),
                Style::default().fg(Color::Cyan),
            ),
            Span::raw("participants"),
        ]),
    ]);

    if app.list.get_selected().is_some() {
        text.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Selected participant: "),
                Span::styled(app.get_highlighted_name(), Style::default().fg(Color::Cyan)),
            ]),
        ]);
    }

    text.extend(vec![
        Line::from(""),
        Line::from(Span::raw("Use ⬆ / ⬇ to select.")),
        Line::from(""),
        Line::from(Span::raw("Use BACKSPACE to remove.")),
    ]);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title("Details")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center),
        chunks[1],
    );
}

pub fn render_tab_3<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    if app.spin_winner.is_some() {
        let mut modal_text = Text::from("");

        modal_text.extend(vec![Line::from(Span::styled(
            format!("{} 🎉", app.spin_winner.clone().unwrap().name),
            Style::default().fg(Color::Green),
        ))]);

        let block = Paragraph::new(modal_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("The winner is")
                .padding(Padding {
                    left: 2,
                    right: 2,
                    top: 0,
                    bottom: 0,
                }),
        );

        let area2 = centered_rect(20, 15, area);
        frame.render_widget(Clear, area2); //this clears out the background
        frame.render_widget(block, area2);
    } else {
        let mut text = Text::from("");

        if app.spinning {
            text.extend(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "*spinning wheel noises*",
                    Style::default().fg(Color::Green),
                )),
                Line::from(""),
                Line::from(Span::raw(format!("🎲 {}", app.spin_counter))),
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    app.get_random().name,
                    Style::default().fg(Color::Cyan),
                )),
            ]);
        } else {
            text.extend(vec![
                Line::from(""),
                Line::from(Span::raw("Ready to roll!")),
            ]);
        }

        let block = Paragraph::new(text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Spin the wheel"),
        );

        frame.render_widget(block, area)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
