// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
use sc_cli::{
	Error, GenerateCmd, GenerateNodeKeyCmd, InspectKeyCmd, InspectNodeKeyCmd, SignCmd, VanityCmd,
	VerifyCmd,
};

#[derive(Debug, Parser)]
#[clap(
	name = "subkey",
	author = "Parity Team <admin@parity.io>",
	about = "Utility for generating and restoring with Substrate keys"
)]
pub enum Subkey {
	/// Generate a random node libp2p key, save it to file or print it to stdout
	/// and print its peer ID to stderr.
	GenerateNodeKey(GenerateNodeKeyCmd),

	/// Generate a random account
	///
	/// todo x: 生成助记词/私钥
	///
	Generate(GenerateCmd),

	////////////////////////////////////////////////////////////////////////////////
	/// Gets a public key and a SS58 address from the provided Secret URI
	Inspect(InspectKeyCmd),

	/// Print the peer ID corresponding to the node key in the given file
	InspectNodeKey(InspectNodeKeyCmd),

	////////////////////////////////////////////////////////////////////////////////
	/// Sign a message, with a given (secret) key.
	Sign(SignCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(VanityCmd),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(VerifyCmd),
}

////////////////////////////////////////////////////////////////////////////////

/// Run the subkey command, given the appropriate runtime.
///
/// todo x:
///
pub fn run() -> Result<(), Error> {
	match Subkey::parse() {
		///
		/// todo x:
		///
		Subkey::GenerateNodeKey(cmd) => cmd.run(),

		///
		/// todo x: 生成助记词
		///
		Subkey::Generate(cmd) => cmd.run(),
		Subkey::Inspect(cmd) => cmd.run(),
		Subkey::InspectNodeKey(cmd) => cmd.run(),
		Subkey::Vanity(cmd) => cmd.run(),
		Subkey::Verify(cmd) => cmd.run(),
		Subkey::Sign(cmd) => cmd.run(),
	}
}
