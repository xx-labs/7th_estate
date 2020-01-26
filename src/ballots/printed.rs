//! # Printed Ballot Information
//!
//! `printed` contains type aliases to identify information that is
//! suitable for printing on the printed ballots.

use super::{BallotSerial, VoteCode, ChoiceValue};

pub type PrintedBallotSerial = BallotSerial;
pub type PrintedVoteCode = VoteCode;
pub type PrintedChoiceValue = ChoiceValue;
