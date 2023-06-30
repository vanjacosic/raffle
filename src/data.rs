use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    pub winner: bool,
}

pub fn read_participants_from_file() -> Result<Vec<Participant>, Box<dyn Error>> {
    let file = File::open("participants.txt")?;
    let lines = BufReader::new(file).lines();

    let results: Vec<Participant> = lines
        .into_iter()
        .map(|p| -> Participant {
            Participant {
                name: p.unwrap(),
                winner: false,
            }
        })
        .collect();

    Ok(results)
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
