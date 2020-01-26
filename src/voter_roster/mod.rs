//! Module file for the Voter Roster.

use std::path::Path;
use crate::Result;

pub mod voter_roster;
pub use voter_roster::*;

pub mod restricted;
pub use restricted::*;

pub mod voter_roster_file;
pub use voter_roster_file::*;

pub mod restricted_file;
pub use restricted_file::*;
