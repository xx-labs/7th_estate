//! # Tagged Ballot Information
//!
//! `tagged` ballots are those suitable for posting as part of the column
//! planes. The decoy ballots have been appropriately marked so their votes
//! will not be counted as part of the tally.

use std::collections::HashSet;
use std::convert::From;
use strum_macros::Display;
use super::{Serialize, Deserialize};
use super::{BallotSerial, VoteCode, ChoiceValue};
use crate::cryptography::csprng::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaggedSerial {
    serial: BallotSerial,
    is_decoy: bool
}

#[derive(Debug, Display, Clone, Copy, Serialize, Deserialize)]
pub enum TaggedChoiceValue { For, Against, Decoy }

#[derive(Debug, Clone, Copy)]
pub struct TaggedBallotChoice {
    serial: TaggedSerial,
    votecode: VoteCode,
    choice: TaggedChoiceValue
}

#[allow(dead_code)]
pub struct TaggedBallot {
    serial: TaggedSerial,
    choice1: TaggedBallotChoice,
    choice2: TaggedBallotChoice
}

impl From<ChoiceValue> for TaggedChoiceValue {
    fn from(value: ChoiceValue) -> Self {
        match value {
            ChoiceValue::For => TaggedChoiceValue::For,
            ChoiceValue::Against => TaggedChoiceValue::Against
        }
    }
}

type ListOfTaggedBallots = Vec<TaggedBallot>;

pub fn string_from_taggedchoicevalue(choice: &TaggedChoiceValue) -> String {
    match choice {
        TaggedChoiceValue::For => "For".to_owned(),
        TaggedChoiceValue::Against => "Against".to_owned(),
        TaggedChoiceValue::Decoy => "Decoy".to_owned()
    }
}

pub fn string_from_taggedchoicevalue_padded(choice: &TaggedChoiceValue) -> String {
    const TCV_PADDED_LENGTH: usize = 7;
    format!("{:width$}", string_from_taggedchoicevalue(choice), width=TCV_PADDED_LENGTH).to_owned()
}


pub fn generate_decoy_serials(seed: CSPRNGSeed, num_decoys: usize, num_ballots: usize) -> Vec<BallotSerial> {
    let mut rng = CSPRNG::from_csprng_seed(seed);
    let mut set = HashSet::new();
    while set.len() < num_decoys {
        set.insert(rng.gen_range(0, num_ballots));
    }
    let mut decoys: Vec<BallotSerial> = set.iter().map(|&n| n).collect();
    decoys.sort();
    decoys
}

pub fn tag_serials(serials: &Vec<BallotSerial>, decoys: &Vec<BallotSerial>) -> Vec<TaggedSerial> {
    serials.iter()
        .map(|&serial| {
            TaggedSerial {
                serial: serial,
                is_decoy: decoys.contains(&serial)
            }
        }).collect::<Vec<TaggedSerial>>()
}

pub fn generate_tagged_ballots(serials: &Vec<TaggedSerial>, votecodes: &Vec<VoteCode>) -> ListOfTaggedBallots {
    assert!((2 * serials.len()) <= votecodes.len(),
        "Too many vote codes supplied.");
    assert!((2 * serials.len()) >= votecodes.len(),
        "Too many ballot serials supplied.");
    let for_choices = serials.iter().zip(votecodes.iter().step_by(2))
        .map(|(&serial, &votecode)| {
            TaggedBallotChoice {
                serial: serial,
                votecode: votecode,
                choice: match serial.is_decoy {
                    true => TaggedChoiceValue::Decoy,
                    false => TaggedChoiceValue::For
                }
            }
        }).collect::<Vec<TaggedBallotChoice>>();
    let against_choices = serials.iter().zip(votecodes.iter().step_by(2))
        .map(|(&serial, &votecode)| {
            TaggedBallotChoice {
                serial: serial,
                votecode: votecode,
                choice: match serial.is_decoy {
                    true => TaggedChoiceValue::Decoy,
                    false => TaggedChoiceValue::Against
                }
            }
        }).collect::<Vec<TaggedBallotChoice>>();
    for_choices.iter().zip(against_choices.iter())
        .map(|(&for_choice, &against_choice)| {
            assert!(for_choice.serial == against_choice.serial,
                "Cannot generate ballot with mismatched serials.");
            TaggedBallot {
                serial: for_choice.serial,
                choice1: for_choice,
                choice2: against_choice
            }
        }).collect::<ListOfTaggedBallots>()
}
