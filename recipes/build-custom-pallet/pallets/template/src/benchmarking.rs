//! Benchmarking setup for pallet-template
//!
//! This module contains benchmarks for the template pallet.
//! Benchmarks are used to estimate the computational weight of extrinsics.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn store_something() {
        let caller: T::AccountId = whitelisted_caller();
        let value: u32 = 100;

        #[extrinsic_call]
        store_something(RawOrigin::Signed(caller), value);

        assert_eq!(Something::<T>::get(), Some(value));
    }

    #[benchmark]
    fn set_balance() {
        let account: T::AccountId = whitelisted_caller();
        let balance: u32 = 1000;

        #[extrinsic_call]
        set_balance(RawOrigin::Root, account.clone(), balance);

        assert_eq!(BalanceOf::<T>::get(&account), balance);
    }

    #[benchmark]
    fn increment() {
        let caller: T::AccountId = whitelisted_caller();
        // Setup: store an initial value
        Something::<T>::put(50u32);

        #[extrinsic_call]
        increment(RawOrigin::Signed(caller));

        assert_eq!(Something::<T>::get(), Some(51));
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
