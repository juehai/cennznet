// Copyright 2019-2020 Parity Technologies (UK) Ltd. and Centrality Investments Ltd.
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

//! Test accounts and signing helpers

use cennznet_primitives::types::{AccountId, AssetId, Balance, FeeExchange, Index};
use cennznet_runtime::{CheckedExtrinsic, SignedExtra, UncheckedExtrinsic};
use codec::Encode;
use sp_keyring::AccountKeyring;
use sp_runtime::generic::Era;

/// Alice's account id.
pub fn alice() -> AccountId {
	AccountKeyring::Alice.into()
}

/// Bob's account id.
pub fn bob() -> AccountId {
	AccountKeyring::Bob.into()
}

/// Charlie's account id.
pub fn charlie() -> AccountId {
	AccountKeyring::Charlie.into()
}

/// Dave's account id.
pub fn dave() -> AccountId {
	AccountKeyring::Dave.into()
}

/// Eve's account id.
pub fn eve() -> AccountId {
	AccountKeyring::Eve.into()
}

/// Ferdie's account id.
pub fn ferdie() -> AccountId {
	AccountKeyring::Ferdie.into()
}

/// Returns transaction extra.
pub fn signed_extra(
	nonce: Index,
	extra_fee: Balance,
	fee_exchange: Option<FeeExchange<AssetId, Balance>>,
) -> SignedExtra {
	(
		frame_system::CheckSpecVersion::new(),
		frame_system::CheckTxVersion::new(),
		frame_system::CheckGenesis::new(),
		frame_system::CheckEra::from(Era::Immortal),
		frame_system::CheckNonce::from(nonce),
		frame_system::CheckWeight::new(),
		crml_transaction_payment::ChargeTransactionPayment::from(extra_fee, fee_exchange),
	)
}

/// Sign given `CheckedExtrinsic`.
pub fn sign(xt: CheckedExtrinsic, spec_version: u32, tx_version: u32, genesis_hash: [u8; 32]) -> UncheckedExtrinsic {
	match xt.signed {
		Some((signed, extra)) => {
			let payload = (
				xt.function,
				extra.clone(),
				spec_version,
				tx_version,
				genesis_hash,
				genesis_hash,
			);
			let key = AccountKeyring::from_account_id(&signed).unwrap();
			let signature = payload
				.using_encoded(|b| {
					if b.len() > 256 {
						key.sign(&sp_io::hashing::blake2_256(b))
					} else {
						key.sign(b)
					}
				})
				.into();
			UncheckedExtrinsic {
				signature: Some((signed, signature, extra)),
				function: payload.0,
			}
		}
		None => UncheckedExtrinsic {
			signature: None,
			function: xt.function,
		},
	}
}
