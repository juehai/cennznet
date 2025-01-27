// Copyright 2018-2021 Parity Technologies (UK) Ltd. and Centrality Investments Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! A set of constant values used in substrate runtime.

/// TestNet Asset IDs.
pub mod asset {
	use cennznet_primitives::types::AssetId;

	pub const CENNZ_ASSET_ID: AssetId = 16000;
	pub const CPAY_ASSET_ID: AssetId = 16001;
	pub const NEXT_ASSET_ID: AssetId = 17000;

	pub const STAKING_ASSET_ID: AssetId = CENNZ_ASSET_ID;
	pub const SPENDING_ASSET_ID: AssetId = CPAY_ASSET_ID;
}

pub mod config {
	// arbitrary keys for storing offchain config.
	// for consistency expect
	// 4 byte key for prefix and 8 byte key for subkeys

	/// offchain storage config key for eth http URI
	pub const ETH_HTTP_URI: [u8; 8] = *b"ETH_HTTP";
}

/// Money matters.
pub mod currency {
	use cennznet_primitives::types::Balance;
	/// The smallest denomination of any currency
	pub const WEI: Balance = 1;
	/// The smallest denomination of a 4dp currency
	pub const MICROS: Balance = WEI;
	/// The dollar denomination of a 4dp currency
	pub const DOLLARS: Balance = 10_000;
}

/// Time.
pub mod time {
	use cennznet_primitives::types::{BlockNumber, Moment};

	/// Since BABE is probabilistic this is the average expected block time that
	/// we are targetting. Blocks will be produced at a minimum duration defined
	/// by `SLOT_DURATION`, but some slots will not be allocated to any
	/// authority and hence no block will be produced. We expect to have this
	/// block time on average following the defined slot duration and the value
	/// of `c` configured for BABE (where `1 - c` represents the probability of
	/// a slot being empty).
	/// This value is only used indirectly to define the unit constants below
	/// that are expressed in blocks. The rest of the code should use
	/// `SLOT_DURATION` instead (like the Timestamp pallet for calculating the
	/// minimum period).
	///
	/// If using BABE with secondary slots (default) then all of the slots will
	/// always be assigned, in which case `MILLISECS_PER_BLOCK` and
	/// `SLOT_DURATION` should have the same value.
	///
	/// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
	pub const MILLISECS_PER_BLOCK: Moment = 5000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;

	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// 1 in 4 blocks (on average, not counting collisions) will be primary BABE blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	#[cfg(not(feature = "integration_config"))]
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 10 * MINUTES;
	#[cfg(feature = "integration_config")]
	pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 1 * MINUTES;

	pub const EPOCH_DURATION_IN_SLOTS: u64 = {
		const SLOT_FILL_RATE: f64 = MILLISECS_PER_BLOCK as f64 / SLOT_DURATION as f64;

		(EPOCH_DURATION_IN_BLOCKS as f64 * SLOT_FILL_RATE) as u64
	};

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	#[cfg(not(feature = "integration_config"))]
	pub const SESSIONS_PER_ERA: sp_staking::SessionIndex = 144;
	#[cfg(feature = "integration_config")]
	pub const SESSIONS_PER_ERA: sp_staking::SessionIndex = 2;
}
