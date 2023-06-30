use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    pub winner: bool,
}

pub fn read_participants_from_file() -> Result<Vec<Participant>, Box<dyn Error>> {
    let file = File::open("participants.txt")?;
    let lines = BufReader::new(file).lines();

    let participants: Vec<Participant> = lines
        .into_iter()
        .map(|p| -> Participant {
            Participant {
                name: p.unwrap(),
                winner: false,
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
