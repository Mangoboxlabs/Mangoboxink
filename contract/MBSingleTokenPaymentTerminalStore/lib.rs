#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
pub use self::mb_SingleTokenPaymentTerminalStore::{
    MBSingleTokenPaymentTerminalStore
};
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod mb_SingleTokenPaymentTerminalStore {
    use MBFundingCycleStore::MBFundingCycleStore;
    use MBDirectory::MBDirectory;
    use ink_prelude::vec::Vec;
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    const _MAX_FIXED_POINT_FIDELITY: u64 = 18;

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
  @member number The funding cycle number for the cycle's project. Each funding cycle has a number that is an increment of the cycle that directly preceded it. Each project's first funding cycle has a number of 1.
  @member configuration The timestamp when the parameters for this funding cycle were configured. This value will stay the same for subsequent funding cycles that roll over from an originally configured cycle.
  @member basedOn The `configuration` of the funding cycle that was active when this cycle was created.
  @member start The timestamp marking the moment from which the funding cycle is considered active. It is a unix timestamp measured in seconds.
  @member duration The number of seconds the funding cycle lasts for, after which a new funding cycle will start. A duration of 0 means that the funding cycle will stay active until the project owner explicitly issues a reconfiguration, at which point a new funding cycle will immediately start with the updated properties. If the duration is greater than 0, a project owner cannot make changes to a funding cycle's parameters while it is active – any proposed changes will apply to the subsequent cycle. If no changes are proposed, a funding cycle rolls over to another one with the same properties but new `start` timestamp and a discounted `weight`.
  @member weight A fixed point number with 18 decimals that contracts can use to base arbitrary calculations on. For example, payment terminals can use this to determine how many tokens should be minted when a payment is received.
  @member discountRate A percent by how much the `weight` of the subsequent funding cycle should be reduced, if the project owner hasn't configured the subsequent funding cycle with an explicit `weight`. If it's 0, each funding cycle will have equal weight. If the number is 90%, the next funding cycle will have a 10% smaller weight.
  @member ballot An address of a contract that says whether a proposed reconfiguration should be accepted or rejected. It can be used to create rules around how a project owner can change funding cycle parameters over time.
  @member metadata Extra data that can be associated with a funding cycle.
*/
    pub struct MBFundingCycle {
        number: u64,
        configuration: u64,
        basedOn: u64,
        start: u64,
        duration: u64,
        weight: u64,
        discountRate: u64,
        ballot: AccountId,
        metadata: u64,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
     @member token The token the payment was made in.
     @member value The amount of tokens that was paid, as a fixed point number.
     @member decimals The number of decimals included in the value fixed point number.
     @member currency The expected currency of the value.
   **/
    pub struct MBTokenAmount {
        token: AccountId,
        value: u128,
        decimals: u64,
        metadata: u128,
    }

    #[ink(storage)]
    pub struct MBSingleTokenPaymentTerminalStore {
        ///The directory of terminals and controllers for projects.
        directory: AccountId,
        ///The contract storing all funding cycle configurations.
        fundingCycleStore: AccountId,
        ///   The contract that exposes price feeds.
        prices: AccountId,
        ///  The amount of tokens that each project has for each terminal, in terms of the terminal's token.
        balanceOf: StorageHashMap<(AccountId, u64), u128>,
        ///  The amount of funds that a project has distributed from its limit during the current funding cycle for each terminal, in terms of the distribution limit's currency.
        usedDistributionLimitOf: StorageHashMap<(AccountId, u64, u64), u128>,
        /// The amount of funds that a project has used from its allowance during the current funding cycle configuration for each terminal, in terms of the overflow allowance's currency.
        usedOverflowAllowanceOf: StorageHashMap<(AccountId, u64, u64), u128>,
    }

    impl MBSingleTokenPaymentTerminalStore {
        #[ink(constructor)]
        pub fn new(
            _prices:AccountId,
            _directory:AccountId,
            _fundingCycleStore:AccountId
        ) -> Self {
            Self {
                directory: _directory,
                fundingCycleStore: _fundingCycleStore,
                prices:_prices,
                balanceOf: Default::default(),
                usedDistributionLimitOf: Default::default(),
                usedOverflowAllowanceOf: Default::default(),
            }
        }
        /**
        @notice
        Gets the current overflowed amount in a terminal for a specified project.
        @param _terminal The terminal for which the overflow is being calculated.
        @param _projectId The ID of the project to get overflow for.

        @return The current amount of overflow that project has in the specified terminal.
      */
        #[ink(message)]
        pub fn currentOverflowOf(
            &self,
            _terminal: AccountId,
            _projectId: u64,
        ) -> u128 {
            return
                self._overflowDuring(
                    _terminal,
                    _projectId,
                    100,
                );
        }
        /**
    @notice
    Gets the current overflowed amount for a specified project across all terminals.

    @param _projectId The ID of the project to get total overflow for.
    @param _decimals The number of decimals that the fixed point overflow should include.
    @param _currency The currency that the total overflow should be in terms of.

    @return The current total amount of overflow that project has across all terminals.
  */
        #[ink(message)]
        pub fn currentTotalOverflowOf(
            &self,
            _projectId: u64,
            _decimals: u64,
            _currency: u64,
        ) -> u128 {
            return self._currentTotalOverflowOf(_projectId, _decimals, _currency);
        }
        /**
        @notice
        Records newly contributed tokens to a project.
        @param _payer The original address that sent the payment to the terminal.
        @param _amount The amount of tokens being paid. Includes the token being paid, the value, the number of decimals included, and the currency of the amount.
        @param _projectId The ID of the project being paid.
        @param _baseWeightCurrency The currency to base token issuance on.
        @param _beneficiary The specified address that should be the beneficiary of anything that results from the payment.
        @param _memo A memo to pass along to the emitted event, and passed along to the funding cycle's data source.
        @param _metadata Bytes to send along to the data source, if one is provided.

        @return fundingCycle The project's funding cycle during which payment was made.
        @return tokenCount The number of project tokens that were minted, as a fixed point number with 18 decimals.
        @return delegate A delegate contract to use for subsequent calls.
        @return memo A memo that should be passed along to the emitted event.
      */
        #[ink(message)]
        pub fn recordPaymentFrom(
            &mut self,
            _payer: AccountId,
            _amount: u128,
            _projectId: u64,
            _baseWeightCurrency: u64,
            _beneficiary: AccountId,
            _memo: String,
            _metadata: String,
        ) -> (MBFundingCycle, u128, AccountId, String) {
            let mut _weight = 0;
            let fundingCycle = MBFundingCycle {
                number: 1,
                configuration: 1,
                basedOn: 1,
                start: 1,
                duration: 1,
                weight: 1,
                discountRate: 1,
                ballot: AccountId::default(),
                metadata: 0,
            };
            let store_instance: MBFundingCycleStore = ink_env::call::FromAccountId::from_account_id(self.fundingCycleStore);

            _weight = store_instance.getProjectsWeight(_projectId);
            if _amount == 0 { return (fundingCycle, 0, AccountId::default(), _memo); };
            let mut _balanceOf = self.balanceOf.get(&(Self::env().caller(), _projectId)).unwrap_or(&0).clone();
            _balanceOf = _balanceOf + _amount;
            self.balanceOf.insert((Self::env().caller(), _projectId), _balanceOf);
            return (fundingCycle, _amount * _weight as u128 / 1000000000000000000, AccountId::default(), _memo);
        }
        /// @notice
        /// get the balance of the project
        #[ink(message)]
        pub fn getBalanceOf(
            &self,
            _account:AccountId,
            _projectId:u64,
        ) ->u128 {
            self.balanceOf.get(&(_account, _projectId)).unwrap_or(&0).clone()
        }
        /**
    @notice
    Records newly redeemed tokens of a project.

    @param _holder The account that is having its tokens redeemed.
    @param _projectId The ID of the project to which the tokens being redeemed belong.
    @param _tokenCount The number of project tokens to redeem, as a fixed point number with 18 decimals.
    @param _memo A memo to pass along to the emitted event.

    @return fundingCycle The funding cycle during which the redemption was made.
    @return reclaimAmount The amount of terminal tokens reclaimed, as a fixed point number with 18 decimals.
    @return delegate A delegate contract to use for subsequent calls.
    @return memo A memo that should be passed along to the emitted event.
  */
        #[ink(message)]
        pub fn recordRedemptionFor(
            &mut self,
            _holder: AccountId,
            _projectId: u64,
            _tokenCount: u128,
            _memo: String,
        ) -> (MBFundingCycle, u128, AccountId, String) {
            let fundingCycle = MBFundingCycle {
                number: 1,
                configuration: 1,
                basedOn: 1,
                start: 1,
                duration: 1,
                weight: 1,
                discountRate: 1,
                ballot: AccountId::default(),
                metadata: 0,
            };
            let mut _balanceOf = self.balanceOf.get(&(Self::env().caller(), _projectId)).unwrap_or(&0).clone();
            _balanceOf = _balanceOf + 100;
            self.balanceOf.insert((Self::env().caller(), _projectId), _balanceOf);
            return (fundingCycle, 100, AccountId::default(), _memo);
        }
        /**
        @notice
        Records newly distributed funds for a project.

        @dev
        The msg.sender must be an IJBSingleTokenPaymentTerminal.

        @param _projectId The ID of the project that is having funds distributed.
        @param _amount The amount to use from the distribution limit, as a fixed point number.
        @param _currency The currency of the `_amount`. This must match the project's current funding cycle's currency.

        @return fundingCycle The funding cycle during which the distribution was made.
        @return distributedAmount The amount of terminal tokens distributed, as a fixed point number with the same amount of decimals as its relative terminal.
      */
        #[ink(message)]
        pub fn recordDistributionFor(
            &mut self,
            _projectId: u64,
            _amount: u128,
            _currency: u128,
        ) ->u128 {
            let fundingCycle = MBFundingCycle {
                number: 1,
                configuration: 1,
                basedOn: 1,
                start: 1,
                duration: 1,
                weight: 1,
                discountRate: 1,
                ballot: AccountId::default(),
                metadata: 0,
            };
            let mut _newUsedDistributionLimitOf = self.usedDistributionLimitOf.get(&(Self::env().caller(), _projectId, fundingCycle.number)).unwrap_or(&0).clone();
            _newUsedDistributionLimitOf += _amount;

            self.usedDistributionLimitOf.insert((Self::env().caller(), _projectId, fundingCycle.number), _newUsedDistributionLimitOf);
            let mut _balanceOf = self.balanceOf.get(&(Self::env().caller(), _projectId)).unwrap_or(&0).clone();
            _balanceOf = _balanceOf + 100;
            self.balanceOf.insert((Self::env().caller(), _projectId), _balanceOf);
            _newUsedDistributionLimitOf
        }
        /**
   @notice
   Records newly used allowance funds of a project.


   @param _projectId The ID of the project to use the allowance of.
   @param _amount The amount to use from the allowance, as a fixed point number.
   @param _currency The currency of the `_amount`. Must match the currency of the overflow allowance.

   @return fundingCycle The funding cycle during which the overflow allowance is being used.
   @return usedAmount The amount of terminal tokens used, as a fixed point number with the same amount of decimals as its relative terminal.
 */
        #[ink(message)]
        pub fn recordUsedAllowanceOf(
            &mut self,
            _projectId: u64,
            _amount: u128,
            _currency: u128,
        ) ->u128 {
            let fundingCycle = MBFundingCycle {
                number: 1,
                configuration: 1,
                basedOn: 1,
                start: 1,
                duration: 1,
                weight: 1,
                discountRate: 1,
                ballot: AccountId::default(),
                metadata: 0,
            };
            let mut _newUsedOverflowAllowanceOf = self.usedOverflowAllowanceOf.get(&(Self::env().caller(), _projectId, fundingCycle.configuration)).unwrap_or(&0).clone();
            _newUsedOverflowAllowanceOf+=_amount;
            self.usedOverflowAllowanceOf.insert((Self::env().caller(), _projectId, fundingCycle.configuration),_newUsedOverflowAllowanceOf);
            let mut _balanceOf = self.balanceOf.get(&(Self::env().caller(), _projectId)).unwrap_or(&0).clone();
            _balanceOf = _balanceOf + 100;
            self.balanceOf.insert((Self::env().caller(), _projectId), _balanceOf);
            _newUsedOverflowAllowanceOf

        }
        /**
        @notice
        Records newly added funds for the project.

        @param _projectId The ID of the project to which the funds being added belong.
        @param _amount The amount of terminal tokens added, as a fixed point number with the same amount of decimals as its relative terminal.
      */
        #[ink(message)]
        pub fn recordAddedBalanceFor(
            &mut self,
            _projectId:u64,
            _amount:u128
        ) ->bool {
            let mut _balanceOf = self.balanceOf.get(&(Self::env().caller(), _projectId)).unwrap_or(&0).clone();
            _balanceOf = _balanceOf + _amount;
            self.balanceOf.insert((Self::env().caller(), _projectId), _balanceOf);
            true
        }

        fn _currentTotalOverflowOf(
            &self,
            _projectId: u64,
            _decimals: u64,
            _currency: u64,
        ) -> u128 {
            return 0;
        }
        fn _overflowDuring(
            &self,
            _terminal: AccountId,
            _projectId: u64,
            _balanceCurrency: u128,
        ) -> u128 {
            let _balanceOf = self.balanceOf.get(&(_terminal, _projectId)).unwrap_or(&0).clone();
            if _balanceOf == 0 { return 0; }
            // todo cross contract
            /*(uint256 _distributionLimit, uint256 _distributionLimitCurrency) = IJBController(
                directory.controllerOf(_projectId)
            ).distributionLimitOf(_projectId, _fundingCycle.configuration, _terminal, _terminal.token());*/
            let _distributionLimit = 100;
            let _distributionLimitCurrency = 0;
            let _usedDistributionLimitOf = self.usedDistributionLimitOf.get(&(_terminal, _projectId, 1)).unwrap_or(&0).clone();
            let mut _distributionLimitRemaining = _distributionLimit - _usedDistributionLimitOf;
            if _distributionLimitRemaining != 0 && _distributionLimitCurrency != _balanceCurrency {
                _distributionLimitRemaining = _distributionLimitRemaining * 10000000000000000000 / 10;
            }
            if _balanceOf > _distributionLimitRemaining { return _balanceOf - _distributionLimitRemaining; } else { return 0; };
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn currentOverflowOf_works() {
            let mp = MBSingleTokenPaymentTerminalStore::new();
            assert!(mp.currentOverflowOf(AccountId::default(),1) == 0);
        }
        #[ink::test]
        fn recordPaymentFrom_works() {
            let mut mp = MBSingleTokenPaymentTerminalStore::new();
            let (_fundingCycle, _number, _token, _meta) = mp.recordPaymentFrom(AccountId::default(),0,1,1,AccountId::default(),String::from("test"),String::from("test"));
            assert!(_fundingCycle.number == 1);
        }
        #[ink::test]
        fn recordRedemptionFor_works() {
            let mut mp = MBSingleTokenPaymentTerminalStore::new();
            let (_fundingCycle, _number, _token, _meta) = mp.recordRedemptionFor(AccountId::default(),1,1,String::from("test"));
            assert!(_fundingCycle.number == 1);
        }
        #[ink::test]
        fn recordDistributionFor_works() {
            let mut mp = MBSingleTokenPaymentTerminalStore::new();
            assert!(mp.recordDistributionFor(1,1,1) == true);
        }
        #[ink::test]
        fn recordUsedAllowanceOf_works() {
            let mut mp = MBSingleTokenPaymentTerminalStore::new();
            assert!(mp.recordUsedAllowanceOf(1,1,1) == true);
        }
        #[ink::test]
        fn recordAddedBalanceFor_works() {
            let mut mp = MBSingleTokenPaymentTerminalStore::new();
            assert!(mp.recordAddedBalanceFor(1,1) == true);

        }
    }
}
