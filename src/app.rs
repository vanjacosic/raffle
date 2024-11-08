use rand::Rng;
use ratatui::widgets::ListState;
use std::{error, path::Path, vec};

use crate::data::{self, Participant};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    // Tabs
    pub tabs: StatefulTabs,

    // Lists
    pub all_participants: StatefulList<Participant>,
    pub all_winners: Vec<Participant>,

    // Spinner
    pub is_spinning: bool,
    pub spin_counter: usize,
    pub spin_winner: Option<Participant>,
}

impl Default for App {
    fn default() -> Self {
        Self::new(Path::new("participants.txt"))
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(path: &Path) -> Self {
        let tab_titles = vec!["Home".to_string(), "Participants".to_string()];

        let participants = data::read_participants_from_file(path).expect("Failed to read file");

        Self {
            running: true,
            tabs: StatefulTabs::new(tab_titles),
            all_participants: StatefulList::new(participants),
            all_winners: Vec::new(),
            is_spinning: false,
            spin_counter: 0,
            spin_winner: None,
        }
    }

    /// Set running to false to quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Handles the tick event of the terminal
    pub fn tick(&mut self) {
        self.spin_round()
    }

    pub fn start_spin(&mut self) {
        if self.all_participants.items.is_empty() {
            return;
        }

        let participant_count = self.all_participants.items.len();

        let min_spins = participant_count * 3;
        let max_spins = participant_count * 6;

        let mut rng = rand::thread_rng();
        let random_spins = rng.gen_range(min_spins..max_spins);

        self.spin_counter = random_spins;
        self.spin_winner = None;
        self.is_spinning = true;
    }

    pub fn spin_round(&mut self) {
        if !self.is_spinning {
            return;
        }

        if self.spin_counter > 0 {
            self.all_participants.next();
            self.spin_counter -= 1;
            return;
        }

        if let Some(winner) = &mut self.all_participants.get_selected() {
            winner.is_winner = true;

            self.spin_winner = Some(winner.clone());
            self.all_winners.push(winner.clone());

            self.stop_spin();
        }
    }

    pub fn stop_spin(&mut self) {
        self.is_spinning = false;
    }

    pub fn reset_spin(&mut self) {
        self.stop_spin();
        self.spin_counter = 0;
        self.spin_winner = None;
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

    pub fn get_selected(&mut self) -> Option<&mut T> {
        match self.state.selected() {
            Some(index) => Some(&mut self.items[index]),
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
