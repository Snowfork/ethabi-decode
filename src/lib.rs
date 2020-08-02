// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be 
// copied, modified, or distributed except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod decoder;
mod errors;
mod event;
mod param;
mod std;
mod token;

pub use crate::{
	decoder::decode,
	errors::{Error, Result},
	event::{Event, Param},
	param::ParamType,
	token::Token,
};

pub use ethereum_types::Address;
pub use ethereum_types::H256;
pub use ethereum_types::U256;

/// ABI word.
pub type Word = [u8; 32];
