use crate::data::{self, Participant, StatefulList};
use rand::Rng;
use std::{error, vec};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    // spin
    pub spinning: bool,

    pub spin_counter: usize,

    pub spin_winner: Option<Participant>,

    // state
    pub list: StatefulList<Participant>,

    // modal
    pub show_modal: bool,

    // tabs
    pub tabs: StatefulTabs,
}

impl Default for App {
    fn default() -> Self {
        // println!("{}", std::env::current_dir().unwrap().display());

        let tab_titles = vec![
            "Home".to_string(),
            "Participants".to_string(),
            "Raffle".to_string(),
        ];

        let participants = data::read_participants_from_file().expect("Failed to read file");

        Self {
            running: true,
            spinning: false,
            spin_counter: 50,
            spin_winner: None,
            tabs: StatefulTabs::new(tab_titles),
            list: StatefulList::new(participants),
            show_modal: false,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /// Set running to false to quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn start(&mut self) {
        self.spin_winner = None;
        self.spin_counter = 50;
        self.spinning = true;
    }

    pub fn get_random(&mut self) -> Participant {
        let mut rng = rand::thread_rng();

        let random_index = rng.gen_range(0..self.list.items.len());

        if self.spin_counter > 0 {
            self.spin_counter -= 1;
            self.list.items[random_index].clone()
        } else {
            let winner = self.list.items[random_index].clone();
            self.spin_winner = Some(winner.clone());
            self.spin_counter = 200;
            self.spinning = false;
            winner
        }
    }

    pub fn get_highlighted_name(&mut self) -> String {
        match self.list.get_selected() {
            Some(pers) => pers.name,
            None => String::from("None"),
        }
    }
}

#[derive(Debug)]
pub struct StatefulTabs {
    pub titles: Vec<String>,
    pub active: usize,
}

impl StatefulTabs {
    pub fn new(titles: Vec<String>) -> StatefulTabs {
        StatefulTabs { titles, active: 0 }
    }

    pub fn next_tab(&mut self) {
        self.active = (self.active + 1) % self.titles.len();
    }

    pub fn prev_tab(&mut self) {
        if self.active > 0 {
            self.active -= 1;
        } else {
            self.active = self.titles.len() - 1;
        }
    }
}
