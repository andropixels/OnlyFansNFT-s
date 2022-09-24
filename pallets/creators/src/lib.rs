#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]

pub struct Creator<AccoundId,BoundedString> {
    address:AccoundId,
    name:BoundedString,
    kyc:BoundedString,
    Subscribers:Vec<T::AccoundId>
    SubscriberCount:u64
}


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type CreatorId: Parameter
        + AtLeast32BitUnsigned
        + Default
        + Copy
        + MaxEncodedLen;

        #[pallet::constant]
		type StringLimit: Get<u32>;


	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	// #[pallet::storage]
	// #[pallet::getter(fn creators)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// pub type Creators<T> = StorageValue<_, u32>;


    #[pallet::storage]
	pub(super) type Creators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CreatorId,
		Creator<T::AccountId, BoundedVec<u8, T::StringLimit>>,
	>;

#[pallet::storage]
#[pallet::getter(fn next_creator_id)]
 pub type NextCreatorId<T:Config> = StorageValue<_, T::CreatorId>;



	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CreatorRegistered(T::CreatorId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		NoneValue,

		StorageOverflow,
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(10_000 )]
		pub fn register_creator(origin: OriginFor<T>, address: T::AccountId,name:Vec<u8>,kyc:Vec<u8>) -> DispatchResult {
		
			let who = ensure_signed(origin)?;

			
            let creator_id = Self::next_creator_id();

            
		let bounded_name: BoundedVec<u8, T::StringLimit> =
        name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

        let bounded_kyc: BoundedVec<u8, T::StringLimit> =
        name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

        let creator = Creator{
            address:address,
            name:bounded_name,
            kyc:bounded_kyc,
            Subscribers:vec![],
            SubscriberCount:0

        }

		    Creators::<T>::insert(creator_id, creator);



			
			Self::deposit_event(Event::SomethingStored(creator_id, creator));
		
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 )]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
