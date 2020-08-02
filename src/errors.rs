// Copyright 2015-2020 Parity Technologies
// Copyright 2020 Snowfork
//
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// http://www.apache.org/licenses/LICENSE-2.0>. This file may not be 
// copied, modified, or distributed except according to those terms.

/// Ethabi result type
pub type Result<T> = crate::std::Result<T, Error>;

/// Ethabi errors
#[derive(Debug)]
pub enum Error {
	/// Invalid entity such as a bad function name.
	InvalidName,
	/// Invalid data.
	InvalidData,
	/// Integer parsing error.
	ParseInt,
	/// UTF-8 parsing error.
	Hex(hex::FromHexError),
	/// Other errors.
	Other,
}

impl From<uint::FromDecStrErr> for Error {
	fn from(err: uint::FromDecStrErr) -> Self {
		use uint::FromDecStrErr::*;
		match err {
			InvalidCharacter => Error::Other,
			InvalidLength => Error::Other,
		}
	}
}
impl From<hex::FromHexError> for Error {
	fn from(err: hex::FromHexError) -> Self {
		Error::Hex(err)
	}
}
