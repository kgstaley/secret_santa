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
}

impl Participant {
    pub fn new(index: usize, name: String) -> Self {
        Self { index, name }
    }
}

pub fn build_participants_from_file(path: PathBuf) -> Result<Vec<Participant>> {
    let f = File::open(&path).expect("could not read file");
    let reader = BufReader::new(f);

    let mut participants: Vec<Participant> = vec![];
    let mut n = 1;
    for line in reader.lines() {
        let content = line?;
        let participant = Participant::new(n, content);
        participants.push(participant);
        n += 1;
    }

    Ok(participants)
}
