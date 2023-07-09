use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    pub is_winner: bool,
}

impl fmt::Display for Participant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_winner {
            write!(f, "🎁 {}", self.name)
        } else {
            write!(f, "{}", self.name)
        }
    }
}

pub fn read_participants_from_file() -> Result<Vec<Participant>, Box<dyn Error>> {
    let file = File::open("participants.txt")?;
    let lines = BufReader::new(file).lines();

    let participants: Vec<Participant> = lines
        .into_iter()
        .map(|p| -> Participant {
            Participant {
                name: p.unwrap(),
                is_winner: false,
            }
        })
        .collect();

    Ok(participants)
}

//pub fn get_participants() -> Result<Vec<Participant>, Box<dyn error::Error>> {
//     let participant_list: Vec<Participant> = vec![
//         Participant {
//             name: String::from("Alice"),
//             winner: false,
//         },
//         Participant {
//             name: String::from("Bob"),
//             winner: false,
//         },
//         Participant {
//             name: String::from("Mallory"),
//             winner: false,
//         },
//         Participant {
//             name: String::from("Ken"),
//             winner: false,
//         },
//     ];

//     Ok(participant_list)
// }
