//! # Ballot Module
//!
//! `ballots` contains only the information needed to manage a poll.
//! For example, the poll question is not included. The poll choices
//! are limited to For and Against.

use serde::{Serialize, Deserialize};

pub mod untagged;
pub use untagged::*;

pub mod tagged;
pub use tagged::*;

pub mod printed;
pub use printed::*;

pub mod print;
pub use print::*;

use std::io::ErrorKind;
use std::path::Path;
use std::fs::DirBuilder;

