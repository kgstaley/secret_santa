use std::path::PathBuf;

use clap::Parser;
use eyre::Result;

mod matcher;
mod participant;

use crate::matcher::make_matches;
use crate::participant::build_participants_from_file;

// Search for a pattern in the file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    // the path to to file to read
    path: PathBuf,
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

#[cfg(test)]
mod tests {
    use crate::participant::Participant;

    use super::*;

    #[test]
    fn all_participants_accounted_for() {
        // build the participants vec
        let mut participants: Vec<Participant> = vec![];
        for n in 1..20 {
            participants.push(Participant {
                index: n,
                name: uuid::Uuid::new_v4().to_string(),
            })
        }
        // generate matches
        let matches = make_matches(&participants);
        let mut receivers = matches
            .into_iter()
            .map(|p| p.receiver)
            .collect::<Vec<Participant>>();
        receivers.sort_by(|a, b| a.index.cmp(&b.index));
        // assert that all participants are accounted for in match
        assert_eq!(participants.len(), receivers.len());
        assert_eq!(participants, receivers);
    }
}
