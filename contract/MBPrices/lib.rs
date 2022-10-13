#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
pub use self::mb_Prices::{
    MBPrices
};
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod mb_Prices {
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    /**
    _currency The currency units the feed's resulting price is in terms of.
    _base The base currency unit being priced by the feed.
  */
    #[ink(storage)]
    pub struct MBPrices {
        feedFor:StorageHashMap<(u128, u128), AccountId>
    }

    impl MBPrices {
        #[ink(constructor)]
        pub fn new(
        ) -> Self {
            Self {
                feedFor: Default::default(),
            }
        }
        /**
        @param _currency The currency units the resulting price is in terms of.
        @param _base The base currency unit being priced.
        @param _decimals The number of decimals the returned fixed point price should include.
        @return The price of the currency in terms of the base, as a fixed point number with the specified number of decimals.
      */
        #[ink(message)]
        pub fn priceFor(
            &self,
            _currency: u128,
            _base: u128,
            _decimals: u128
        ) -> u128 {
            // if _currency == _base {
            return 10 ** &_decimals;
            // }
            // let feed = self.feedFor.get(&(_currency,_base)).unwrap_or(&AccountId::default()).clone();
            // if feed != AccountId::default() {
            //     return 1;
            // } else {
            //     return 0;
            // }
        }
        /**
          @notice
          Add a price feed for a currency in terms of the provided base currency.

          @dev
          Current feeds can't be modified.

          @param _currency The currency units the feed's resulting price is in terms of.
          @param _base The base currency unit being priced by the feed.
          @param _feed The price feed being added.
        */
        #[ink(message)]
        pub fn addFeedFor(
            &mut self,
            _currency: u128,
            _base: u128,
            _feed: AccountId
        ) -> bool {
            let feed = self.feedFor.get(&(_currency,_base)).unwrap_or(&AccountId::default()).clone();
            assert!(feed == AccountId::default(),"FEED IS EXISTS");
            self.feedFor.insert((_currency, _base),_feed);
            true
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn init_works() {
            let mut mp = MBPrices::new();
            assert!(mp.addFeedFor(1,1,AccountId::default()) == true);
            assert!(mp.priceFor(1,0,0) == 0);
        }
    }
}
