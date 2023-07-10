use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Padding,
};

pub const LOGO: &str = r#"

██████╗     █████╗    ███████╗███████╗██╗        ███████╗   
██╔══██╗   ██╔══██╗   ██╔════╝██╔════╝██║        ██╔════╝   
██████╔╝   ███████║   █████╗  █████╗  ██║        █████╗     
██╔══██╗   ██╔══██║   ██╔══╝  ██╔══╝  ██║        ██╔══╝     
██║  ██║██╗██║  ██║██╗██║██╗  ██║██╗  ███████╗██╗███████╗██╗
╚═╝  ╚═╝╚═╝╚═╝  ╚═╝╚═╝╚═╝╚═╝  ╚═╝╚═╝  ╚══════╝╚═╝╚══════╝╚═╝

"#;

pub const FERRIS: &str = r#"
 _____________________________
/         Made for the        \
\  Copenhagen Rust Community  /
 +---------------------------+
        \
         \
            _~^~^~_    |🇩🇰
        \) /  o o  \ (/|
          '_   -   _'
          / '-----' \
"#;

pub const PADDING: Padding = Padding {
    left: 2,
    right: 2,
    top: 1,
    bottom: 1,
};

pub fn action() -> Style {
    Style::default().fg(Color::LightCyan)
}

pub fn action_highlight() -> Style {
    action()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED)
}

pub fn winner() -> Style {
    action().fg(Color::LightGreen)
}

pub fn winner_highlight() -> Style {
    winner()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED)
}

pub fn key() -> Style {
    action().add_modifier(Modifier::UNDERLINED)
}

pub fn spin() -> Style {
    Style::default().fg(Color::LightYellow)
}

pub fn spin_highlight() -> Style {
    spin()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::REVERSED)
}

pub fn orange() -> Style {
    Style::default().fg(Color::Indexed(214))
}
