//! # Seventh Estate Poll System

use log::*;
#[macro_use] extern crate slice_as_array;

const NUMBER_OF_PLANES: usize = 50;

type Exception = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Exception>;

pub mod cryptography;
use cryptography::*;

pub mod secrets;
use secrets::*;

pub mod poll_configuration;
use poll_configuration::*;

pub mod planes;
use planes::*;

pub mod voter_roster;
use voter_roster::*;

pub mod summands;
use summands::*;

pub mod voter_selection;

pub mod blockchain;

pub mod ballots;
use ballots::*;

pub mod subcommands;

