use rand::{seq::SliceRandom, thread_rng};

use crate::participant::Participant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    pub giver: Participant,
    pub receiver: Participant,
}

impl Match {
    pub fn new(giver: Participant, receiver: Participant) -> Self {
        Self { giver, receiver }
    }
}

pub trait Matcher {
    fn make_matches(participants: &Vec<Participant>) -> Vec<Match>;
}

impl Matcher for Match {
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
