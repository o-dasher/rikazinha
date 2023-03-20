use crate::error::RikaError;

pub mod osu;
pub mod fun;

pub type CommandReturn = Result<(), RikaError>;
