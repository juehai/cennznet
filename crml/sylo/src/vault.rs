// Copyright (C) 2019 Centrality Investments Limited
// This file is part of CENNZnet.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
use srml_support::{dispatch::Vec, StorageMap};
use {system, system::ensure_signed};

extern crate primitives;
extern crate runtime_primitives;
extern crate sr_io;

pub const KEYS_MAX: usize = 100;

pub trait Trait: system::Trait {}

pub type VaultKey = Vec<u8>;
pub type VaultValue = Vec<u8>;

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn upsert_value(origin, key: VaultKey, value: VaultValue) {
			let user_id = ensure_signed(origin)?;

			ensure!(<Vault<T>>::get(&user_id).len() < KEYS_MAX, "Can not store more than maximum amount of keys");

			Self::upsert(user_id, key, value);
		}

		fn delete_values(origin, keys: Vec<VaultKey>) {
			let user_id = ensure_signed(origin)?;

			Self::delete(user_id, keys);
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as SyloVault {
		pub Vault get(values): map T::AccountId => Vec<(VaultKey, VaultValue)>;
	}
}

impl<T: Trait> Module<T> {
	pub fn upsert(user_id: T::AccountId, key: VaultKey, value: VaultValue) {
		let mut values = <Vault<T>>::get(&user_id);

		match values.iter().enumerate().find(|(_, item)| item.0 == key) {
			None => values.push((key, value)),
			Some((i, _)) => values[i] = (key, value),
		}

		<Vault<T>>::insert(user_id, values)
	}

	pub fn delete(user_id: T::AccountId, keys: Vec<VaultKey>) {
		let remaining_values: Vec<(VaultKey, VaultValue)> = <Vault<T>>::get(&user_id)
			.into_iter()
			.filter(|item| keys.iter().find(|key_to_remove| &&item.0 == key_to_remove).is_none())
			.collect();

		<Vault<T>>::insert(user_id, remaining_values)
	}
}

#[cfg(test)]
mod tests {
	use self::sr_io::with_externalities;
	use super::*;
	use mock::{new_test_ext, Origin, Test};
	use primitives::H256;

	impl Trait for Test {}
	type Vault = Module<Test>;

	#[test]
	fn should_upsert_values() {
		with_externalities(&mut new_test_ext(), || {
			let key_0 = b"0".to_vec();
			let value_0 = b"1".to_vec();

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_0.clone(),
				value_0.clone()
			));

			assert_eq!(
				Vault::values(H256::from_low_u64_be(1)),
				vec![(key_0.clone(), value_0.clone())]
			);

			let key_1 = b"01".to_vec();
			let value_1 = b"10".to_vec();

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_1.clone(),
				value_1.clone()
			));

			assert_eq!(
				Vault::values(H256::from_low_u64_be(1)),
				vec![(key_0, value_0), (key_1, value_1)]
			);
		});
	}

	#[test]
	fn should_replace_existing_keys() {
		with_externalities(&mut new_test_ext(), || {
			let key_0 = b"0".to_vec();
			let value_0 = b"1".to_vec();
			let value_1 = b"01".to_vec();

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_0.clone(),
				value_0.clone()
			));

			assert_eq!(Vault::values(H256::from_low_u64_be(1)), vec![(key_0.clone(), value_0)]);

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_0.clone(),
				value_1.clone()
			));

			assert_eq!(Vault::values(H256::from_low_u64_be(1)), vec![(key_0, value_1)]);
		});
	}

	#[test]
	fn should_delete_keys() {
		with_externalities(&mut new_test_ext(), || {
			let key_0 = b"0".to_vec();
			let key_1 = b"1".to_vec();
			let value_0 = b"01".to_vec();

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_0.clone(),
				value_0.clone()
			));

			assert_ok!(Vault::upsert_value(
				Origin::signed(H256::from_low_u64_be(1)),
				key_1.clone(),
				value_0.clone()
			));

			assert_eq!(
				Vault::values(H256::from_low_u64_be(1)),
				vec![(key_0.clone(), value_0.clone()), (key_1.clone(), value_0)]
			);

			assert_ok!(Vault::delete_values(
				Origin::signed(H256::from_low_u64_be(1)),
				vec![key_0, key_1]
			));

			assert_eq!(Vault::values(H256::from_low_u64_be(1)), vec![]);
		});
	}
}
