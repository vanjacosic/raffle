use ratatui::style::{Color, Modifier, Style};

pub fn variable() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn action() -> Style {
    Style::default().fg(Color::Cyan)
}

pub fn key() -> Style {
    action().add_modifier(Modifier::UNDERLINED)
}

pub fn active() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED)
}

pub fn pad_text(string: &str) -> String {
    format!(" {} ", string)
}
