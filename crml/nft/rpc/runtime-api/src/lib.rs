// Copyright 2020-2021 Parity Technologies (UK) Ltd. and Centrality Investments Ltd.
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

//! Runtime API definition required by NFT RPC extensions.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use crml_nft::{CollectionId, CollectionInfo, Config, Listing, ListingId, SerialNumber, SeriesId, TokenId, TokenInfo};
use sp_std::prelude::*;

sp_api::decl_runtime_apis! {
	/// The RPC API to interact with NFT module
	pub trait NftApi<AccountId, T> where
		AccountId: Codec,
		T: Config,
	{
		/// Find all the tokens owned by `who` in a given collection
		fn collected_tokens(
			collection_id: CollectionId,
			who: AccountId,
		) -> Vec<TokenId>;

		/// Get collection info from a given collection
		fn collection_info(collection_id: CollectionId) -> Option<CollectionInfo<AccountId>>;

		/// Get information of given token
		fn token_info(
		   collection_id: CollectionId,
		   series_id: SeriesId,
		   serial_number: SerialNumber,
		) -> TokenInfo<AccountId>;

		fn collection_listings(
			collection_id: CollectionId,
			offset: u128,
			limit: u16,
		) -> (Option<u128>, Vec<(ListingId, Listing<T>)>);
	}
}
