//! # Column 2 Entries

use super::*;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum Vote {
    Voted,
    NotVoted
}

#[derive(Debug, Clone, Serialize)]
pub enum Column2Entry {
    Empty,
    Entry(Vote)
}

