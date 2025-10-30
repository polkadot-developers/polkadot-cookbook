#![cfg_attr(not(feature = "std"), no_std)]

//! # Template Pallet
//!
//! A simple FRAME pallet template demonstrating basic functionality.
//!
//! ## Overview
//!
//! This pallet provides a simple example of:
//! - Storage items
//! - Dispatchable functions (extrinsics)
//! - Events
//! - Errors
//!
//! Replace this with your actual pallet implementation.

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        #[allow(deprecated)]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Storage item for a simple value.
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;

    /// Storage map for account balances.
    #[pallet::storage]
    #[pallet::getter(fn balance_of)]
    pub type BalanceOf<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// Events are a way of reporting specific conditions and circumstances to external entities.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a value is stored.
        SomethingStored { value: u32, who: T::AccountId },
        /// Event emitted when a balance is set.
        BalanceSet { who: T::AccountId, balance: u32 },
    }

    /// Errors inform users why an extrinsic failed.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Store a value in storage.
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn store_something(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <Something<T>>::put(value);
            Self::deposit_event(Event::SomethingStored { value, who });

            Ok(())
        }

        /// Set the balance for an account.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
        pub fn set_balance(
            origin: OriginFor<T>,
            account: T::AccountId,
            balance: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <BalanceOf<T>>::insert(&account, balance);
            Self::deposit_event(Event::BalanceSet {
                who: account,
                balance,
            });

            Ok(())
        }

        /// Increment the stored value by 1.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().reads_writes(1, 1))]
        pub fn increment(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let current = <Something<T>>::get().ok_or(Error::<T>::NoneValue)?;
            let new_value = current.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

            <Something<T>>::put(new_value);
            Self::deposit_event(Event::SomethingStored {
                value: new_value,
                who,
            });

            Ok(())
        }
    }
}
