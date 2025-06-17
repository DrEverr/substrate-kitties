use super::*;
use frame::prelude::*;
use frame::traits::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		use frame_system::Pallet as FrameSystem;
		let payload = (
			FrameSystem::<T>::parent_hash(),
			FrameSystem::<T>::block_number(),
			FrameSystem::<T>::extrinsic_index(),
			CountForKitties::<T>::get(),
		);
		BlakeTwo256::hash_of(&payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty::<T> { dna, owner: owner.clone() };
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let count = CountForKitties::<T>::get();
		let new_count = count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyKittiesOwned)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::put(new_count);

		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::NoKitty)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwned);

		kitty.owner = to.clone();
		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyKitties)?;
		let mut from_owned = KittiesOwned::<T>::get(&from);
		if let Some(index) = from_owned.iter().position(|&i| i == kitty_id) {
			from_owned.swap_remove(index);
		} else {
			return Err(Error::<T>::NoKitty.into());
		}

		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);
		Kitties::<T>::insert(kitty_id, kitty);
		Self::deposit_event(Event::<T>::Transferred { from, to, kitty: kitty_id });
		Ok(())
	}
}
