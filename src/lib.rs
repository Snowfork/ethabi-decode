#![no_std]

#[cfg(feature = "std")]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

mod util;
mod param;
mod token;
mod errors;
mod decoder;
mod event;

pub use crate::{
	decoder::decode,
	errors::{Error, Result},
	param::ParamType,
    token::Token,
    event::{Event, Param}
};


pub use ethereum_types::H256;
pub use ethereum_types::U256;
pub use ethereum_types::Address;

/// ABI word.
pub type Word = [u8; 32];
