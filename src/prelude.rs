//! Crate prelude


pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern, mostly used for
pub struct W<T>(pub T);

pub use std::format as f;