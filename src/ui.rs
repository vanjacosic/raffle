use crate::{app::App, styles};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();

    let panes = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(size);

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|title| Line::from(format!(" {} ", title)))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default())
        .select(app.tabs.active)
        .style(styles::action())
        .highlight_style(styles::action_highlight());

    frame.render_widget(tabs, panes[0]);

    match app.tabs.active {
        0 => render_tab_1(app, frame, panes[1]),
        1 => render_tab_2(app, frame, panes[1]),
        _ => (),
    };
}

pub fn render_tab_1<B: Backend>(_app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let mut text = Text::from(styles::LOGO);

    text.patch_style(styles::winner());

    text.extend(vec![
        Line::from(""),
        Line::from(Span::styled(
            "( Rapidly Assembled Ferris Fortune Locator Engine )",
            styles::secondary(),
        )),
        Line::from(""),
        Line::from(""),
        Line::from("Made for the"),
        Line::from(Span::styled(
            "Copenhagen Rust Community 🦀🧡",
            styles::orange(),
        )),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::raw("Press "),
            Span::styled("Tab", styles::key()),
            Span::raw(" to continue or "),
            Span::styled("Q", styles::key()),
            Span::raw(" to exit."),
        ]),
    ]);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().padding(styles::PADDING)),
        area,
    );
}

pub fn render_tab_2<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(area);

    render_list(app, frame, layout[0]);

    let layout_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(layout[1]);

    render_spin(app, frame, layout_right[0]);
    render_status(app, frame, layout_right[1]);
}

pub fn render_list<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let mut list_items: Vec<_> = app
        .all_participants
        .items
        .iter()
        .map(|participant| {
            let item = ListItem::new(format!(" {} ", participant));

            if participant.is_winner {
                item.style(styles::winner())
            } else {
                item.style(styles::orange())
            }
        })
        .collect();

    if app.all_participants.items.is_empty() {
        list_items.push(ListItem::new(" No participants. "))
    }

    let list = List::new(list_items)
        .block(
            Block::default()
                .title(" All participants ")
                .borders(Borders::ALL)
                .padding(Padding {
                    left: 0,
                    right: 0,
                    top: 1,
                    bottom: 1,
                }),
        )
        .highlight_style({
            let winner_higlighted = {
                match app.all_participants.get_selected() {
                    Some(selected) => {
                        if let Some(spin_winner) = &app.spin_winner {
                            spin_winner.clone_into(selected);
                            true
                        } else {
                            false
                        }
                    }
                    None => false,
                }
            };

            if winner_higlighted {
                styles::winner_highlight()
            } else if app.is_spinning {
                styles::spin_highlight()
            } else {
                styles::action_highlight()
            }
        });

    frame.render_stateful_widget(list, area, &mut app.all_participants.state);
}

pub fn render_spin<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    // State: Empty
    if app.all_participants.items.is_empty() {
        return;
    }

    // State: Ready
    let mut modal_content: Paragraph<'_>;
    let mut modal_text = Text::from("\nReady to roll 🎲");

    // State: Spinning
    if app.is_spinning {
        modal_text = Text::from(vec![
            Line::from(""),
            Line::from(Span::styled("*spinning wheel noises*", styles::spin())),
        ]);
    }

    modal_content = Paragraph::new(modal_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("  Spin the wheel  ")
                .title_alignment(Alignment::Center)
                .padding(styles::PADDING),
        );

    // State: Winner found
    if let Some(winner) = &app.spin_winner {
        modal_text = Text::from(vec![
            Line::from(""),
            Line::from(winner.clone().to_string()),
            Line::from(""),
            Line::from("🎉🎉🎉"),
        ]);

        modal_content = Paragraph::new(modal_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("  The winner is  ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .style(styles::winner()),
            );
    }

    let modal = create_modal(40, 50, area);
    frame.render_widget(Clear, modal);
    frame.render_widget(modal_content, modal);
}

pub fn render_status<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>, area: Rect) {
    let split_pane = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area);

    let participant_count: usize = app.all_participants.items.len();

    let mut status_text = Text::from(vec![
        Line::from(vec![
            Span::styled(format!("✋ {}", participant_count), styles::orange()),
            Span::raw(" participants"),
        ]),
        Line::from(""),
    ]);

    let chance: f32 = 1_f32 / participant_count as f32;
    let percentage = format!("{:.1}%", chance * 100_f32);

    status_text.extend(vec![Line::from(vec![
        Span::styled(format!("🍀 {}", percentage), styles::orange()),
        Span::raw(" chance to win"),
    ])]);

    if app.is_spinning {
        status_text.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(format!("🎲 {}", app.spin_counter), styles::orange()),
                Span::raw(" rotations left"),
            ]),
        ]);
    }

    frame.render_widget(
        Paragraph::new(status_text).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" Status ")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .padding(styles::PADDING),
        ),
        split_pane[0],
    );

    let winners: Vec<Line> = app
        .all_winners
        .iter()
        .flat_map(|p| {
            vec![
                Line::from(Span::styled(p.to_string(), styles::winner())),
                Line::from(""),
            ]
        })
        .collect();

    frame.render_widget(
        Paragraph::new(winners).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" Winners ")
                .borders(Borders::ALL)
                .padding(styles::PADDING),
        ),
        split_pane[1],
    );

    let help_text = Text::from(vec![
        Line::from(vec![
            Span::styled("S", styles::key()),
            Span::raw(" to start the spin. "),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("R", styles::key()),
            Span::raw(" to reset the spin."),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("⬇", styles::key()),
            Span::raw("  / "),
            Span::styled("⬆", styles::key()),
            Span::raw("  to select list.\n"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Backspace", styles::key()),
            Span::raw(" to remove. "),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Q", styles::key()),
            Span::raw(" to quit. "),
        ]),
    ]);

    frame.render_widget(
        Paragraph::new(help_text).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .padding(styles::PADDING),
        ),
        split_pane[2],
    );
}

// Modal window
fn create_modal(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
