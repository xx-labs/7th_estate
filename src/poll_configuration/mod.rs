//! Module for poll configuration information and files.

use serde::{Serialize, Deserialize};
use crate::cryptography::{Base64String, AEADString};
use crate::ballots::VoteCode;

pub mod complete;
pub use complete::*;

pub mod secured;
pub use secured::*;

pub mod new;
pub use new::*;
