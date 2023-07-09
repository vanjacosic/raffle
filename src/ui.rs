use crate::{app::App, styles};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs, Wrap},
    Frame,
};

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
        .map(|title| Line::from(Span::raw(styles::pad_text(title))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default())
        .select(app.tabs.active)
        .style(styles::action())
        .highlight_style(styles::active());

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
        Line::from("Made for the"),
        Line::from(Span::styled(
            "Copenhagen Rust Community 🦀🧡",
            Style::default().fg(Color::LightRed),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("Ctrl-C", styles::key()),
            Span::raw(" or "),
            Span::styled("q", styles::key()),
            Span::raw(" to exit."),
        ]),
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
            ),
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
        .map(|participant| {
            let item = ListItem::new(participant.to_string());

            if participant.is_winner {
                item.style(styles::winner())
            } else {
                item.style(styles::action())
            }
        })
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
        .highlight_style(styles::active());

    frame.render_stateful_widget(item_list, chunks[0], &mut app.list.state);

    let mut text = Text::from(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(app.list.items.len().to_string(), styles::variable()),
            Span::raw(" participants"),
        ]),
    ]);

    if app.list.get_selected().is_some() {
        text.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::raw("Selected participant: "),
                Span::styled(app.get_highlighted_name(), styles::variable()),
            ]),
        ]);
    }

    text.extend(vec![
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::raw("Use "),
            Span::styled("⬇", styles::key()),
            Span::raw(" / "),
            Span::styled("⬆", styles::key()),
            Span::raw(" to select."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Use "),
            Span::styled("Backspace", styles::key()),
            Span::raw(" to remove."),
        ]),
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
            ),
        chunks[1],
    );
}

pub fn render_tab_3<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    if let Some(winner) = &app.spin_winner {
        let modal_text = Text::from(vec![
            Line::from(winner.clone().to_string()),
            Line::from(""),
            Line::from("🎉🎉🎉"),
        ]);

        let modal = Paragraph::new(modal_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("The winner is")
                    .title_alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Green))
                    .padding(Padding {
                        left: 2,
                        right: 2,
                        top: 1,
                        bottom: 1,
                    }),
            );

        let inner_area = centered_rect(25, 20, area);
        frame.render_widget(Clear, inner_area);
        frame.render_widget(modal, inner_area);
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
                Line::from("Will it be"),
                Line::from(Span::styled(app.get_random().name, styles::variable())),
                Line::from("?"),
            ]);
        } else {
            text.extend(vec![
                Line::from(""),
                Line::from(Span::raw("Ready to roll.")),
                Line::from(""),
                Line::from(Span::raw("Press `s` to start the spin.")),
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

// Modal window
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
