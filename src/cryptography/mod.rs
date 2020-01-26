//! Module for abstract cryptographic primitive operations.

use log::debug;
use serde::{Serialize, Deserialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64String(pub String);

pub mod kdf;
pub use kdf::*;

pub mod authenticated_encryption;
pub use authenticated_encryption::*;

pub mod signature;
pub use signature::*;

pub mod secret_sharing;
pub use secret_sharing::*;

pub mod secured_file;
pub use secured_file::*;

pub mod csprng;
pub use csprng::*;

pub mod fast_dice_roller;
pub use fast_dice_roller::*;

mod endian;
