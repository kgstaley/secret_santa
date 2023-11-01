use eyre::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Participant {
    pub index: usize,
    pub name: String,
    pub email: Option<String>,
}

impl Participant {
    pub fn new(index: usize, name: String, email: Option<String>) -> Self {
        Self { index, name, email }
    }
}

pub trait BuildFromFile {
    fn build_participants_from_file(path: PathBuf) -> Result<Vec<Participant>>;
}

impl BuildFromFile for Participant {
    fn build_participants_from_file(path: PathBuf) -> Result<Vec<Participant>> {
        let f = File::open(&path).expect("could not read file");
        let reader = BufReader::new(f);

        let mut participants: Vec<Participant> = vec![];
        let mut n = 1;
        for line in reader.lines() {
            let contents = line?;
            let split = contents.split(",").collect::<Vec<&str>>();
            let name = split[0].trim().to_string();
            let mut email = None;
            if split.len() > 1 {
                email = Some(split[1].trim().to_string())
            }
            let participant = Participant::new(n, name, email);
            participants.push(participant);
            n += 1;
        }

        Ok(participants)
    }
}
