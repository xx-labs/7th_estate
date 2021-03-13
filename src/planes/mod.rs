//! # Column Planes
//!

use serde::{Serialize, Deserialize};
use crate::ballots::{BallotSerial, VoteCode};
/*
use crate::ballots::{
    string_from_ballotserial,
    string_from_votecode,
    string_from_taggedchoicevalue,
    string_from_taggedchoicevalue_padded
};
*/
use crate::cryptography::*;

pub mod record;
pub use record::*;

pub mod column1;
pub use column1::*;

pub mod column2;
pub use column2::*;

pub mod column3;
pub use column3::*;

pub mod filter;
pub use filter::*;


#[derive(Debug)]
pub struct Plane {
    pub rows: Vec<PlaneRecord>
}

#[derive(Debug)]
pub struct PermutedPlane {
    pub rows: Vec<PlaneRecord>
}

impl Plane {
    pub fn mark_rows(self: &Self, voted: &Vec<usize>) -> Self {
        Plane {
            rows: self.rows.iter().enumerate()
                .map(|(n, row)| {
                    if voted.contains(&n) { row.mark_voted() }
                    else { row.mark_not_voted() }
                })
                .collect::<Vec<PlaneRecord>>()
        }
    }

    pub fn decrypt(self: &Self, filter: &PlaneFilter) -> Self {
        assert!(self.len() == filter.len(),
            "Plane and Filter must have the same number of rows.");
        Plane {
            rows: self.rows.iter().zip(filter.rows.iter())
                .map(|(prec, frec)| { prec.decrypt(frec) })
                .collect::<Vec<PlaneRecord>>()
        }
    }

    pub fn len(self: &Self) -> usize { self.rows.len() }

    pub fn permute(self: &Self, permutation: &Vec<usize>) -> PermutedPlane {
        PermutedPlane {
            rows: permutation.iter().map(|&n| self.rows[n].clone()).collect()
        }
    }
}

