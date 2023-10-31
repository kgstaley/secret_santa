use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use eyre::Result;
use rand::{seq::SliceRandom, thread_rng};

// Search for a pattern in the file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // the path to to file to read
    path: PathBuf,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Participant {
    pub index: usize,
    pub name: String,
}

impl Participant {
    pub fn new(index: usize, name: String) -> Self {
        Self { index, name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Match {
    giver: Participant,
    receiver: Participant,
}

impl Match {
    pub fn new(giver: Participant, receiver: Participant) -> Self {
        Self { giver, receiver }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let participants = build_participants_from_file(args.path)?;
    let matches = make_matches(&participants);

    println!("\n=== matches are in! ===\n");
    matches
        .clone()
        .into_iter()
        .for_each(|m| println!("matched: {:?}\n", m));
    Ok(())
}

fn build_participants_from_file(path: PathBuf) -> Result<Vec<Participant>> {
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

fn make_matches(participants: &Vec<Participant>) -> Vec<Match> {
    let mut matches: Vec<Match> = vec![];
    let mut remaining_match_pool = participants.clone();
    // keep iterating while there are still matches remaining in pool
    while remaining_match_pool.len() > 0 {
        for p in participants {
            if let Some(target) = remaining_match_pool.clone().choose(&mut thread_rng()) {
                if p == target {
                    continue;
                } else {
                    // create the match and append to matches
                    let new_match = Match::new(p.clone(), target.clone());
                    matches.push(new_match);
                    // remove target from remaining pool
                    remaining_match_pool.retain(|item| !item.eq(target));
                }
            }
        }
    }
    matches
}

}
