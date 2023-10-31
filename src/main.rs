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

#[derive(Debug, Clone)]
struct Match {
    giver: Participant,
    receiver: Participant,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let f = File::open(&args.path).expect("could not read file");
    let reader = BufReader::new(f);

    let mut matches: Vec<Match> = vec![];
    let mut participants: Vec<Participant> = vec![];
    let mut n = 1;
    for line in reader.lines() {
        let content = line?;
        let participant = Participant {
            index: n,
            name: content,
        };
        participants.push(participant);
        n += 1;
    }

    let mut remaining_pool = participants.clone();
    for p in &participants {
        if let Some(target) = remaining_pool.clone().choose(&mut thread_rng()) {
            if p == target {
                continue;
            } else {
                // create the match and append to matches
                let new_match = Match {
                    giver: p.clone(),
                    receiver: target.clone(),
                };
                matches.push(new_match);
                // remove target from remaining pool
                remaining_pool.retain(|item| !item.eq(target));
            }
        }
    }

    println!("\n=== matches are in! ===\n");
    matches
        .into_iter()
        .for_each(|m| println!("matched: {:?}\n", m));

    Ok(())
}
