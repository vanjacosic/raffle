use rand::Rng;
use ratatui::widgets::ListState;
use std::{error, vec};

use crate::data::{self, Participant};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const SPIN_ROUNDS: usize = 50;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    // state
    pub list: StatefulList<Participant>,

    // tabs
    pub tabs: StatefulTabs,

    // spin
    pub spinning: bool,

    pub spin_counter: usize,

    pub spin_winner: Option<Participant>,
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
            spin_counter: SPIN_ROUNDS,
            spin_winner: None,
            tabs: StatefulTabs::new(tab_titles),
            list: StatefulList::new(participants),
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
        self.spin_counter = SPIN_ROUNDS;
        self.spinning = true;
    }

    pub fn get_random(&mut self) -> Participant {
        let mut rng = rand::thread_rng();

        let random_index = rng.gen_range(0..self.list.items.len());

        if self.spin_counter > 0 {
            self.spin_counter -= 1;
            self.list.items[random_index].clone()
        } else {
            let winner = &mut self.list.items[random_index];

            winner.is_winner = true;

            self.spin_winner = Some(winner.clone());
            self.spin_counter = SPIN_ROUNDS;
            self.spinning = false;

            winner.clone()
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

#[derive(Debug)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T: std::clone::Clone> StatefulList<T> {
    pub fn new(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn get_selected(&mut self) -> Option<T> {
        match self.state.selected() {
            Some(index) => Some(self.items[index].clone()),
            _ => None,
        }
    }

    pub fn remove(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let Some(i) = self.state.selected() else { return };

        self.items.remove(i);
        self.state.select(None);
    }
}
