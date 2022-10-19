#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;

#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod MBERC20PaymentTerminal {
    use MBSingleTokenPaymentTerminalStore::MBSingleTokenPaymentTerminalStore;
    use MBPrices::MBPrices;
    use MBToken::MBToken;
    use MBTokenStore::MBTokenStore;
    use MBProjects::MBProjects;
    use ink_prelude::vec::Vec;
    use alloc::string::String;

    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    const _PROTOCOL_PROJECT_ID: u64 = 1;
    #[derive(Debug,scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member amount The total amount the fee was taken from, as a fixed point number with the same number of decimals as the terminal in which this struct was created.
      @member fee The percent of the fee, out of MAX_FEE.
      @member feeDiscount The discount of the fee.
      @member beneficiary The address that will receive the tokens that are minted as a result of the fee payment.
    */
    pub struct MBFee {
        amount:u128,
        fee:u64,
        feeDiscount:u64,
        beneficiary:AccountId
    }
    #[derive(Debug,scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member token The token the payment was made in.
      @member value The amount of tokens that was paid, as a fixed point number.
      @member decimals The number of decimals included in the value fixed point number.
      @member currency The expected currency of the value.
    */
    pub struct MBTokenAmount {
         token:AccountId,
        value:u128,
         decimals:u128,
         currency:u128
    }
    #[derive(Debug,scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member value The amount of tokens that was paid, as a fixed point number.
      @member decimals The number of decimals included in the value fixed point number.
      @member currency The expected currency of the value.
    */
    pub struct MBPayRecord {
        value:u128,
        payer:AccountId,
        time:u64
    }
    #[ink(storage)]
    pub struct MBERC20PaymentTerminal {
        _heldFeesOf:StorageHashMap<u64, Vec<MBFee>>,
        payRecords:StorageHashMap<u64, Vec<MBPayRecord>>,
        claimAmount:StorageHashMap<u64, u128>,
        isFeelessAddress:StorageHashMap<AccountId, bool>,
        projects:AccountId,
        directory:AccountId,
        splitsStore:AccountId,
        prices:AccountId,
        store:AccountId,
        feeGauge:AccountId,
        token:AccountId,
        tokenStore:AccountId,
        baseWeightCurrency:u64,
        payoutSplitsGroup:u128,
        decimals:u128,
        currency:u128,
    }

    impl MBERC20PaymentTerminal {
        #[ink(constructor)]
        pub fn new(
            _projects:AccountId,
            _directory:AccountId,
            _splitsStore:AccountId,
            _prices:AccountId,
            _store:AccountId,
            _token:AccountId,
            _tokenStore:AccountId,
        ) -> Self {
            Self {
                _heldFeesOf:Default::default(),
                payRecords:Default::default(),
                claimAmount:Default::default(),
                isFeelessAddress:Default::default(),
                projects:_projects,
                directory:_directory,
                splitsStore:_splitsStore,
                prices:_prices,
                store:_store,
                feeGauge:Default::default(),
                token:_token,
                tokenStore:_tokenStore,
                baseWeightCurrency:0,
                payoutSplitsGroup:0,
                decimals:0,
                currency:0,
            }
        }
        /**
        @notice
        Gets the current overflowed amount in this terminal for a specified project, in terms of ETH.

        @param _projectId The ID of the project to get overflow for.

        @return The current amount of ETH overflow that project has in this terminal, as a fixed point number with 18 decimals.
      */
        #[ink(message)]
        pub fn currentEthOverflowOf(
            &self,
            _projectId: u64
        ) -> u128 {
            if self.store == AccountId::default() || self.prices == AccountId::default() {
                return 0;
            }
            let  store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
            let  prices_instance: MBPrices = ink_env::call::FromAccountId::from_account_id(self.prices);
            let _adjustedOverflow =  store_instance.currentOverflowOf(Self::env().account_id(), _projectId);
            return _adjustedOverflow * 10000000000000000000 / (prices_instance.priceFor(self.currency, 18, self.decimals));
        }
        /**
        @notice
        The fees that are currently being held to be processed later for each project.

        @param _projectId The ID of the project for which fees are being held.

        @return An array of fees that are being held.
      */
        #[ink(message)]
        pub fn heldFeesOf(
            &self,
            _projectId: u64
        ) -> Vec<MBFee> {
            self._heldFeesOf.get(&_projectId).unwrap_or(&Vec::new()).clone()
        }
        /**
        @notice
        Contribute tokens to a project.

        @param _projectId The ID of the project being paid.
        @param _amount The amount of terminal tokens being received, as a fixed point number with the same amount of decimals as this terminal. If this terminal's token is ETH, this is ignored and msg.value is used in its place.
        @param _token The token being paid. This terminal ignores this property since it only manages one token.
        @param _beneficiary The address to mint tokens for and pass along to the funding cycle's data source and delegate.
        @param _minReturnedTokens The minimum number of project tokens expected in return, as a fixed point number with the same amount of decimals as this terminal.
        @param _preferClaimedTokens A flag indicating whether the request prefers to mint project tokens into the beneficiaries wallet rather than leaving them unclaimed. This is only possible if the project has an attached token contract. Leaving them unclaimed saves gas.
        @param _memo A memo to pass along to the emitted event, and passed along the the funding cycle's data source and delegate.  A data source can alter the memo before emitting in the event and forwarding to the delegate.
        @param _metadata Bytes to send along to the data source, delegate, and emitted event, if provided.

        @return The number of tokens minted for the beneficiary, as a fixed point number with 18 decimals.
      */
        #[ink(message)]
        pub fn pay(
            &mut self,
             _projectId:u64,
             _amount:u128,
             _token:AccountId,
             _beneficiary:AccountId,
            _minReturnedTokens:u128,
            _preferClaimedTokens:bool,
             _memo:String,
            _metadata:String
        ) -> bool {
            if self.token == AccountId::default() { return false }
            self._transferFrom(Self::env().caller(), Self::env().account_id(), _amount);
             self._pay(
                    _amount,
                    Self::env().caller(),
                    _projectId,
                    _beneficiary,
                    _minReturnedTokens,
                    _preferClaimedTokens,
                    _memo,
                    _metadata
                );
            true
        }
        /**
       @notice
       Holders can redeem their tokens to claim the project's overflowed tokens, or to trigger rules determined by the project's current funding cycle's data source.

       @param _holder The account to redeem tokens for.
       @param _projectId The ID of the project to which the tokens being redeemed belong.
       @param _tokenCount The number of project tokens to redeem, as a fixed point number with 18 decimals.
       @param _token The token being reclaimed. This terminal ignores this property since it only manages one token.
       @param _minReturnedTokens The minimum amount of terminal tokens expected in return, as a fixed point number with the same amount of decimals as the terminal.
       @param _beneficiary The address to send the terminal tokens to.
       @param _memo A memo to pass along to the emitted event.
       @param _metadata Bytes to send along to the data source, delegate, and emitted event, if provided.

       @return reclaimAmount The amount of terminal tokens that the project tokens were redeemed for, as a fixed point number with 18 decimals.
     */
        #[ink(message)]
        pub fn redeemTokensOf(
            &mut self,
            _holder:AccountId,
             _projectId:u64,
             _tokenCount:u128,
             _token:AccountId,
             _minReturnedTokens:u128,
            _beneficiary:AccountId,
            _memo:String,
        ) -> u128 {
            return
                self._redeemTokensOf(
                    _holder,
                    _projectId,
                    _tokenCount,
                    _minReturnedTokens,
                    _beneficiary,
                    _memo,
                );
        }
        /**
        @notice
        Distributes payouts for a project with the distribution limit of its current funding cycle.
        @param _projectId The ID of the project having its payouts distributed.
        @param _amount The amount of terminal tokens to distribute, as a fixed point number with same number of decimals as this terminal.
        @param _currency The expected currency of the amount being distributed. Must match the project's current funding cycle's distribution limit currency.
        @param _token The token being distributed. This terminal ignores this property since it only manages one token.
        @param _minReturnedTokens The minimum number of terminal tokens that the `_amount` should be valued at in terms of this terminal's currency, as a fixed point number with the same number of decimals as this terminal.
        @param _memo A memo to pass along to the emitted event.

        @return netLeftoverDistributionAmount The amount that was sent to the project owner, as a fixed point number with the same amount of decimals as this terminal.
      */
        #[ink(message)]
        pub fn distributePayoutsOf(
            &mut self,
             _projectId:u64,
             _amount:u128,
             _currency:u128,
             _token:AccountId,
             _minReturnedTokens:u128,
             _memo:String
        ) -> bool{
            return self._distributePayoutsOf(_projectId, _amount, _currency, _minReturnedTokens, _memo);
        }
        /**
        @notice
        Allows a project to send funds from its overflow up to the preconfigured allowance.

        @param _projectId The ID of the project to use the allowance of.
        @param _amount The amount of terminal tokens to use from this project's current allowance, as a fixed point number with the same amount of decimals as this terminal.
        @param _currency The expected currency of the amount being distributed. Must match the project's current funding cycle's overflow allowance currency.
        @param _token The token being distributed. This terminal ignores this property since it only manages one token.
        @param _minReturnedTokens The minimum number of tokens that the `_amount` should be valued at in terms of this terminal's currency, as a fixed point number with 18 decimals.
        @param _beneficiary The address to send the funds to.
        @param _memo A memo to pass along to the emitted event.

        @return netDistributedAmount The amount of tokens that was distributed to the beneficiary, as a fixed point number with the same amount of decimals as the terminal.
      */
        #[ink(message)]
        pub fn useAllowanceOf(
            &mut self,
            _projectId:u64,
            _amount:u128,
            _currency:u128,
            _token:AccountId,
            _minReturnedTokens:u128,
            _beneficiary:AccountId,
            _memo:String
        ) ->bool{
            return self._useAllowanceOf(_projectId, _amount, _currency, _minReturnedTokens, _beneficiary, _memo);
        }
        /**
          @notice
          Get pay records by projects
          @param _projectId The ID of the project to which the funds received belong.
        */
        #[ink(message)]
        pub fn getPayRecords(
            &self,
            _projectId:u64
        ) ->Vec<MBPayRecord> {
            self.payRecords.get(&_projectId).unwrap_or(&Vec::new()).clone()
        }
        /**
        @notice
        Get pay amount by projects
        @param _projectId The ID of the project to which the funds received belong.
      */
        #[ink(message)]
        pub fn getPayAmount(
            &self,
            _projectId:u64
        )->u128{
            let payRecord = self.getPayRecords(_projectId);
            let mut amount = 0;
            for i in payRecord.iter() {
                amount+=i.value
            }
            amount
        }
        /**
           @notice
           Get distributeAmount  by projects
           @param _projectId The ID of the project to which the funds received belong.
         */
        #[ink(message)]
        pub fn distributeAmount(
            &self,
            _projectId:u64
        )->u128{
            return self.getPayAmount(_projectId) - self.claimAmount.get(&_projectId).unwrap_or(&0).clone()
        }
        #[ink(message)]
        pub fn claimDistributeAmount(
            &mut self,
            _projectId:u64,
            _amount:u128
        )->bool{
            assert!(_amount <= self.distributeAmount(_projectId),"Not enough");
            let mut amount = self.claimAmount.get(&_projectId).unwrap_or(&0).clone();
            amount+=_amount;
            self.claimAmount.insert(_projectId,amount);
            self._transferFrom(Self::env().account_id(),Self::env().caller(), _amount);
            true
        }
        /**
        @notice
        Receives funds belonging to the specified project.

        @param _projectId The ID of the project to which the funds received belong.
        @param _amount The amount of tokens to add, as a fixed point number with the same number of decimals as this terminal. If this is an ETH terminal, this is ignored and msg.value is used instead.
        @param _token The token being paid. This terminal ignores this property since it only manages one currency.
        @param _memo A memo to pass along to the emitted event.
        @param _metadata Extra data to pass along to the emitted event.
      */
        #[ink(message)]
        pub fn addToBalanceOf(
            &mut self,
             _projectId:u64,
             _amount:u128,
             _token:AccountId,
            _memo:String,
            _metadata:String
        )->bool{
            if self.token == AccountId::default() { return false}
           self._transferFrom(Self::env().caller(), Self::env().account_id(), _amount);
            let feed = self.isFeelessAddress.get(&Self::env().caller()).unwrap_or(&false).clone();
           self._addToBalanceOf(_projectId, _amount, !feed, _memo, _metadata);
            true
        }
        /**
            @notice
            Receives funds belonging to the specified project.

            @param _projectId The ID of the project to which the funds received belong.
            @param _amount The amount of tokens to add, as a fixed point number with the same number of decimals as this terminal. If this is an ETH terminal, this is ignored and msg.value is used instead.
            @param _shouldRefundHeldFees A flag indicating if held fees should be refunded based on the amount being added.
            @param _memo A memo to pass along to the emitted event.
            @param _metadata Extra data to pass along to the emitted event.
          */
        fn _addToBalanceOf(
            &mut self,
            _projectId:u64,
            _amount:u128,
            _shouldRefundHeldFees:bool,
            _memo:String,
            _metadata:String
        ){
            let mut  store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
            store_instance.recordAddedBalanceFor(_projectId, _amount);
        }
        /**
        @notice
        Allows a project to send funds from its overflow up to the preconfigured allowance.
        @param _projectId The ID of the project to use the allowance of.
        @param _amount The amount of terminal tokens to use from this project's current allowance, as a fixed point number with the same amount of decimals as this terminal.
        @param _currency The expected currency of the amount being distributed. Must match the project's current funding cycle's overflow allowance currency.
        @param _minReturnedTokens The minimum number of tokens that the `_amount` should be valued at in terms of this terminal's currency, as a fixed point number with 18 decimals.
        @param _beneficiary The address to send the funds to.
        @param _memo A memo to pass along to the emitted event.

        @return netDistributedAmount The amount of tokens that was distributed to the beneficiary, as a fixed point number with the same amount of decimals as the terminal.
      */

        fn _useAllowanceOf(
            &mut self,
            _projectId:u64,
            _amount:u128,
            _currency:u128,
            _minReturnedTokens:u128,
            _beneficiary:AccountId,
            _memo:String
        ) ->bool{
            if self.store == AccountId::default() { return false}
            let mut store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
            let _distributedAmount = store_instance.recordUsedAllowanceOf(
                _projectId,
                _amount,
                _currency
            );
            let  projects_instance: MBProjects = ink_env::call::FromAccountId::from_account_id(self.projects);
            let  _projectOwner = projects_instance.owner_of(_projectId);
            let owner = _projectOwner.unwrap_or(AccountId::default());
            self._transferFrom(Self::env().account_id(), owner, _distributedAmount);
            true
        }
        /**
        @notice
        Distributes payouts for a project with the distribution limit of its current funding cycle.
        @param _projectId The ID of the project having its payouts distributed.
        @param _amount The amount of terminal tokens to distribute, as a fixed point number with same number of decimals as this terminal.
        @param _currency The expected currency of the amount being distributed. Must match the project's current funding cycle's distribution limit currency.
        @param _minReturnedTokens The minimum number of terminal tokens that the `_amount` should be valued at in terms of this terminal's currency, as a fixed point number with the same number of decimals as this terminal.
        @param _memo A memo to pass along to the emitted event.

        @return netLeftoverDistributionAmount The amount that was sent to the project owner, as a fixed point number with the same amount of decimals as this terminal.
      */
        fn _distributePayoutsOf(
            &mut self,
            _projectId:u64,
            _amount:u128,
            _currency:u128,
            _minReturnedTokens:u128,
            _memo:String
        ) ->bool{
            if self.store == AccountId::default() { return false}
            let  mut store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
            let  _distributedAmount = store_instance.recordDistributionFor(
                _projectId,
                _amount,
                _currency
            );
            let  projects_instance: MBProjects = ink_env::call::FromAccountId::from_account_id(self.projects);
            let  _projectOwner = projects_instance.owner_of(_projectId);
            let owner = _projectOwner.unwrap_or(AccountId::default());
            self._transferFrom(Self::env().account_id(), owner, _distributedAmount);
            true
        }
        /**
           @notice
           Holders can redeem their tokens to claim the project's overflowed tokens, or to trigger rules determined by the project's current funding cycle's data source.
           @param _holder The account to redeem tokens for.
           @param _projectId The ID of the project to which the tokens being redeemed belong.
           @param _tokenCount The number of project tokens to redeem, as a fixed point number with 18 decimals.
           @param _minReturnedTokens The minimum amount of terminal tokens expected in return, as a fixed point number with the same amount of decimals as the terminal.
           @param _beneficiary The address to send the terminal tokens to.
           @param _memo A memo to pass along to the emitted event.
           @param _metadata Bytes to send along to the data source, delegate, and emitted event, if provided.

           @return reclaimAmount The amount of terminal tokens that the project tokens were redeemed for, as a fixed point number with 18 decimals.
         */
        fn _redeemTokensOf(
            &mut self,
            _holder:AccountId,
            _projectId:u64,
            _tokenCount:u128,
            _minReturnedTokens:u128,
            _beneficiary:AccountId,
            _memo:String,
        )-> u128{
            if self.store == AccountId::default() { return 0}
            if _tokenCount > 0 {
                let mut tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
                let _ret = tokenStore.burnFrom(_holder, _projectId, _tokenCount, false);
            }
            let mut store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
           let (_fundingCycle, reclaimAmount, _delegate, _memo) = store_instance.recordRedemptionFor(
                _holder,
                _projectId,
                _tokenCount,
                _memo,
            );
            // self._transferFrom(Self::env().account_id(), _beneficiary, reclaimAmount);
            return  reclaimAmount;
        }
        /**
        @notice
        Transfers tokens.

        @param _from The address from which the transfer should originate.
        @param _to The address to which the transfer should go.
        @param _amount The amount of the transfer, as a fixed point number with the same number of decimals as this terminal.
      */
        fn _transferFrom(
            &mut self,
             _from:AccountId,
             _to:AccountId,
             _amount:u128
        ){
            let mut token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(self.token);
            if _from == Self::env().account_id()  {
                token_instance.transfer(_to,_amount);
            } else {
                token_instance.transfer_from(_from, _to, _amount);
            }
        }

        /**
        @notice
        Contribute tokens to a project.

        @param _amount The amount of terminal tokens being received, as a fixed point number with the same amount of decimals as this terminal. If this terminal's token is ETH, this is ignored and msg.value is used in its place.
        @param _payer The address making the payment.
        @param _projectId The ID of the project being paid.
        @param _beneficiary The address to mint tokens for and pass along to the funding cycle's data source and delegate.
        @param _minReturnedTokens The minimum number of project tokens expected in return, as a fixed point number with the same amount of decimals as this terminal.
        @param _preferClaimedTokens A flag indicating whether the request prefers to mint project tokens into the beneficiaries wallet rather than leaving them unclaimed. This is only possible if the project has an attached token contract. Leaving them unclaimed saves gas.
        @param _memo A memo to pass along to the emitted event, and passed along the the funding cycle's data source and delegate.  A data source can alter the memo before emitting in the event and forwarding to the delegate.
        @param _metadata Bytes to send along to the data source, delegate, and emitted event, if provided.

        @return beneficiaryTokenCount The number of tokens minted for the beneficiary, as a fixed point number with 18 decimals.
      */
        fn _pay(
            &mut self,
            _amount:u128,
            _payer:AccountId,
            _projectId:u64,
            _beneficiary:AccountId,
            _minReturnedTokens:u128,
            _preferClaimedTokens:bool,
            _memo:String,
            _metadata:String
        )  {
            let mut records = self.payRecords.get(&_projectId).unwrap_or(&Vec::new()).clone();
            let record = MBPayRecord{
                value:_amount,
                payer:_payer,
                time:Self::env().block_timestamp()
            };
            records.push(record);
            self.payRecords.insert(_projectId,records);
            let mut store_instance: MBSingleTokenPaymentTerminalStore = ink_env::call::FromAccountId::from_account_id(self.store);
            store_instance.recordPaymentFrom(
                _payer,
                _amount,
                _projectId,
                self.baseWeightCurrency,
                _beneficiary,
                _memo,
                _metadata
            );
            if _amount > 0 {
                let mut tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
                tokenStore.mintFor(_beneficiary, _projectId, _amount, _preferClaimedTokens);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn currentEthOverflowOf_works() {
            let  MT= MBERC20PaymentTerminal::new();
            assert!(MT.currentEthOverflowOf(1) == 0);
        }
        #[ink::test]
        fn heldFeesOf_works() {
            let MT = MBERC20PaymentTerminal::new();
            let vec:Vec<MBFee> = Vec::new();
            assert!(MT.heldFeesOf(1).len() == vec.len());
        }
        #[ink::test]
        fn pay_works() {
            let mut MT = MBERC20PaymentTerminal::new();
            let  ret  = MT.pay(1,1,AccountId::default(),AccountId::default(),1,false,String::from("test"),String::from("test"));
            assert!(ret == false);
        }
        #[ink::test]
        fn redeemTokensOf_works() {
           let mut MT = MBERC20PaymentTerminal::new();
           let ret =  MT.redeemTokensOf(AccountId::default(),1,1,AccountId::default(),1,AccountId::default(),String::from("test"));
            assert!(ret == 0);
        }
        #[ink::test]
        fn distributePayoutsOf_works() {
            let mut MT = MBERC20PaymentTerminal::new();
            let ret =  MT.distributePayoutsOf(1,1,1,AccountId::default(),1,String::from("test"));
            assert!(ret == false);
        }
        #[ink::test]
        fn useAllowanceOf_works() {
            let mut MT = MBERC20PaymentTerminal::new();
            let ret =  MT.useAllowanceOf(1,1,1,AccountId::default(),1,AccountId::default(),String::from("test"));
            assert!(ret == false);
        }
        #[ink::test]
        fn addToBalanceOf_works() {
            let mut MT = MBERC20PaymentTerminal::new();
            let ret = MT.addToBalanceOf(1,1,AccountId::default(),String::from("test"),String::from("test"));
            assert!(ret == false);
        }

    }

}
