//! # Ballot Information
//!
//! `untagged` ballots are those suitable for printing.
//! They have not been tagged as decoys.

use std::cmp::max;
use super::{Serialize, Deserialize};
use crate::cryptography::csprng::*;
use crate::cryptography::fast_dice_roller::*;

pub type BallotSerial = usize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChoiceValue { For, Against }
pub const CHOICE_VALUES: [ChoiceValue; 2] = [ChoiceValue::For, ChoiceValue::Against];

const VOTE_CODE_NUM_GROUPS: usize = 4;
const VOTE_CODE_GROUP_SIZE: usize = 4;
const VOTE_CODE_NO_PARITY_LENGTH: usize = VOTE_CODE_NUM_GROUPS * VOTE_CODE_GROUP_SIZE;
pub const VOTE_CODE_LENGTH: usize = VOTE_CODE_NUM_GROUPS * (VOTE_CODE_GROUP_SIZE + 1);
const NPVC_MODULUS: u128 = 1_0000_0000_0000_0000;

pub type VoteCodeNoParity = [u8; VOTE_CODE_NO_PARITY_LENGTH];
pub type VoteCode = [u8; VOTE_CODE_LENGTH];


#[derive(Debug, Clone, Copy)]
pub struct BallotChoice {
    pub serial: BallotSerial,
    pub votecode: VoteCode,
    pub choice: ChoiceValue
}

#[derive(Debug)]
pub struct Ballot {
    pub serial: BallotSerial,
    pub choice1: BallotChoice,
    pub choice2: BallotChoice
}

type ListOfBallots = Vec<Ballot>;

pub fn string_from_ballotserial(serial: &BallotSerial, num_ballots: usize) -> String {
    let digits = {
        fn try_n_digits(value: usize, max_value: usize, num_digits: usize) -> usize {
            if value < max_value { num_digits }
            else { try_n_digits(value, 10 * max_value, num_digits + 1) }
        }
        try_n_digits(num_ballots - 1, 10, 1)
    };
    format!("{:0width$}", serial, width=digits)
}

pub fn string_from_votecode(votecode: &VoteCode) -> String {
    let votecode_digits: Vec<String> = votecode.iter()
        .map(|n| format!("{}", n))
        .collect();
    (0..votecode_digits.len()).into_iter()
        .step_by(VOTE_CODE_GROUP_SIZE + 1)
        .map(|base| {
            votecode_digits[base..(base + VOTE_CODE_GROUP_SIZE + 1)].join("")
        })
        .collect::<Vec<String>>()
        .join("-")
        .to_owned()
}

pub fn string_from_choicevalue(choice: &ChoiceValue) -> String {
    match choice {
        ChoiceValue::For => "For".to_owned(),
        ChoiceValue::Against => "Against".to_owned()
    }
}

pub fn generate_votecodes(seed: CSPRNGSeed, count: usize) -> Vec<VoteCode> {
    fn try_generate(seed: CSPRNGSeed, count: usize, num_bytes: usize) -> Option<Vec<usize>> {
        let mut prng = CSPRNG::from_csprng_seed(seed);
        let mut bytes = Vec::<u8>::new();
        bytes.resize_with(num_bytes, || {0});
        prng.fill_bytes(&mut bytes);
        let mut fdr = FastDiceRoller::from_bytes(&bytes);
        let npvotecodes: Vec<Option<u128>> = (0..count).into_iter()
            .map(|_| { fdr.random(NPVC_MODULUS) })
            .collect();
        match npvotecodes.iter().any(|s| s.is_none()) {
            true => None,
            false => Some(npvotecodes.iter().map(|s| s.unwrap() as usize).collect())
        }
    }

    let mut num_bytes: usize = 1024;
    let npvotecodes: Vec<VoteCodeNoParity>;
    loop {
        let maybe_npvotecodes = try_generate(seed, count, num_bytes);
        if let Some(npvcs) = maybe_npvotecodes {
            npvotecodes = npvcs.iter()
                .map(|&npvc| {
                    vcnp_from_vec(
                        (0..VOTE_CODE_NO_PARITY_LENGTH).into_iter()
                            .rev()
                            .map(|n| {
                                let shift = usize::checked_pow(10, n as u32).unwrap();
                                ((npvc / shift) % 10) as u8
                            })
                            .collect::<Vec<u8>>())
                }).collect();
            break;
        }
        num_bytes = num_bytes + 1024;
    }

    npvotecodes.iter()
        .map(|npvc| {
            let mut vc: VoteCode = [0; VOTE_CODE_LENGTH];

            // Copy the no-parity code into the real code.
            (0..vc.len()).into_iter().step_by(VOTE_CODE_GROUP_SIZE + 1)
                .zip((0..npvc.len()).into_iter().step_by(VOTE_CODE_GROUP_SIZE))
                .for_each(|(vcbase, npvcbase)| {
                    (0..VOTE_CODE_GROUP_SIZE).into_iter()
                        .for_each(|n| {
                            vc[vcbase + n] = npvc[npvcbase + n];
                        });
                });

            // Compute the parity digits.
            (0..vc.len()).into_iter().step_by(VOTE_CODE_GROUP_SIZE + 1)
                .for_each(|base| {
                    let parityidx: usize = base + VOTE_CODE_GROUP_SIZE;
                    let sum = vc.iter()
                        .skip(base)
                        .take(VOTE_CODE_GROUP_SIZE)
                        .map(|&x| x as usize)
                        .sum::<usize>();
                    vc[parityidx] = (((10 * VOTE_CODE_GROUP_SIZE) - sum) % 10) as u8;
                });
            vc
        }).collect::<Vec<VoteCode>>()
}

pub fn generate_ballots(serials: &Vec<BallotSerial>, votecodes: &Vec<VoteCode>) -> ListOfBallots {
    assert!((2 * serials.len()) <= votecodes.len(),
        "Too many vote codes supplied.");
    assert!((2 * serials.len()) >= votecodes.len(),
        "Too many ballot serials supplied.");
    let for_choices = serials.iter().zip(votecodes.iter().step_by(2))
        .map(|(&serial, &votecode)| {
            BallotChoice {
                serial: serial,
                votecode: votecode,
                choice: ChoiceValue::For
            }
        }).collect::<Vec<BallotChoice>>();
    let against_choices = serials.iter().zip(votecodes.iter().skip(1).step_by(2))
        .map(|(&serial, &votecode)| {
            BallotChoice {
                serial: serial,
                votecode: votecode,
                choice: ChoiceValue::Against
            }
        }).collect::<Vec<BallotChoice>>();
    for_choices.iter().zip(against_choices.iter())
        .map(|(&for_choice, &against_choice)| {
            assert!(for_choice.serial == against_choice.serial,
                "Cannot generate ballot with mismatched serials.");
            Ballot {
                serial: for_choice.serial,
                choice1: for_choice,
                choice2: against_choice
            }
        }).collect::<ListOfBallots>()
}


pub fn vcnp_from_vec(value: Vec<u8>) -> VoteCodeNoParity {
    let mut vcnp: VoteCodeNoParity = [0; VOTE_CODE_NO_PARITY_LENGTH];
    let copylen = max(VOTE_CODE_NO_PARITY_LENGTH, value.len());
    vcnp.copy_from_slice(&value[0..copylen]);
    vcnp
}

