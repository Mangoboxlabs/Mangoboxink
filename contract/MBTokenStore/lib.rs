#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
pub use self::mbTokenStore::{
    MBTokenStore
};
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]

mod mbTokenStore {
    use MBToken::MBToken;
    use alloc::string::String;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    #[ink(storage)]
    pub struct MBTokenStore {
        projects:AccountId,
        /// Each project's attached token contract.
        tokenOf:StorageHashMap<u64,AccountId>,
        ///The ID of the project that each token belongs to.
        projectOf:StorageHashMap<AccountId,u64>,
        ///    The total supply of unclaimed tokens for each project.
        unclaimedTotalSupplyOf:StorageHashMap<u64,u128>,
        ///    Each holder's balance of unclaimed tokens for each project.
        unclaimedBalanceOf:StorageHashMap<(AccountId, u64), u128>,
        ///    A flag indicating if tokens are required to be issued as claimed for a particular project.
        requireClaimFor:StorageHashMap<u64,bool>,
        /// erc20 HASH
        tokenHash:Hash
    }

    impl MBTokenStore {
        #[ink(constructor)]
        pub fn new(
            _projects:AccountId,
            _tokenHash:Hash,
        ) -> Self {
            Self {
                projects:_projects,
                tokenOf:Default::default(),
                projectOf:Default::default(),
                unclaimedTotalSupplyOf:Default::default(),
                unclaimedBalanceOf:Default::default(),
                requireClaimFor:Default::default(),
                tokenHash:_tokenHash,
            }
        }
        /**
        @notice
        The total supply of tokens for each project, including claimed and unclaimed tokens.

        @param _projectId The ID of the project to get the total token supply of.

        @return totalSupply The total supply of the project's tokens.
      */
        #[ink(message)]
        pub fn totalSupplyOf(
            &self,
            _projectId: u64,
        ) -> u128 {
            let mut totalSupply = self.unclaimedTotalSupplyOf.get(&_projectId).unwrap_or(&0).clone();
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            if _token !=  AccountId::default(){
                let token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(_token);
                totalSupply = totalSupply +  token_instance.total_supply();
            }
            totalSupply
        }
        /**
          @notice
          The total balance of tokens a holder has for a specified project, including claimed and unclaimed tokens.

          @param _holder The token holder to get a balance for.
          @param _projectId The project to get the `_holder`s balance of.

          @return balance The project token balance of the `_holder
        */
        #[ink(message)]
        pub fn balanceOf(
            &self,
            _holder: AccountId,
            _projectId: u64,
        ) -> u128 {
            let mut balance = self.unclaimedBalanceOf.get(&(_holder,_projectId)).unwrap_or(&0).clone();
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            if _token !=  AccountId::default(){
                let token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(_token);
                balance = balance +  token_instance.balance_of(_holder);
            }
            balance
        }
        /**
          @notice
            Issues a project's ERC-20 tokens that'll be used when claiming tokens.

            @dev
            Deploys a project's ERC-20 token contract.

            @param _projectId The ID of the project being issued tokens.
            @param _name The ERC-20's name.
            @param _symbol The ERC-20's symbol.

            @return token The token that was issued.
       */
        #[ink(message)]
        pub fn issueFor(
            &mut self,
            _projectId: u64,
            _name:String,
            _symbol:String
        ) -> AccountId {
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            assert!(_token == AccountId::default(),"PROJECT_ALREADY_HAS_TOKEN");
            let salt = _projectId.to_le_bytes();
            let instance_params = MBToken::new(_name,_symbol)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(self.tokenHash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Erc20` contract");
            self.tokenOf.insert(_projectId,contract_addr);
            self.projectOf.insert(contract_addr,_projectId);
            contract_addr
        }
        /**
         @notice
           Get the token address by project id
           @param _projectId The ID of the project being issued tokens.
           @return token The token address.
      */
        #[ink(message)]
        pub fn getProjectTokenAddress(
            &self,
            _projectId:u64
        ) ->AccountId {
            self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone()
        }
        /**
           @notice
           Mint new project tokens.

           @dev
           Only a project's current controller can mint its tokens.

           @param _holder The address receiving the new tokens.
           @param _projectId The ID of the project to which the tokens belong.
           @param _amount The amount of tokens to mint.
           @param _preferClaimedTokens A flag indicating whether there's a preference for minted tokens to be claimed automatically into the `_holder`s wallet if the project currently has a token contract attached.
         */
        #[ink(message)]
        pub fn mintFor(
            &mut self,
            _holder:AccountId,
            _projectId: u64,
            _amount:u128,
            _preferClaimedTokens:bool,
        ) -> bool {
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            if _token == AccountId::default() {
                return false
            }
            let requireClaimFor = self.requireClaimFor.get(&_projectId).unwrap_or(&false).clone();
            let _shouldClaimTokens = (requireClaimFor || _preferClaimedTokens) &&
                _token != AccountId::default();
           if _shouldClaimTokens {
               let mut token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(_token);
               token_instance.mint_token_by_owner(_holder,_amount);
           }else {
               let balance = self.unclaimedBalanceOf.get(&(_holder,_projectId)).unwrap_or(&0).clone();
               let totalSupply = self.unclaimedTotalSupplyOf.get(&_projectId).unwrap_or(&0).clone();
               self.unclaimedBalanceOf.insert((_holder,_projectId),balance + _amount);
               self.unclaimedTotalSupplyOf.insert(_projectId,totalSupply + _amount);
           }
            true
        }
        /**
            @notice
            Claims internally accounted for tokens into a holder's wallet.

            @dev
            Only a token holder or an operator specified by the token holder can claim its unclaimed tokens.

            @param _holder The owner of the tokens being claimed.
            @param _projectId The ID of the project whose tokens are being claimed.
            @param _amount The amount of tokens to claim.
      */
        #[ink(message)]
        pub fn claimFor(
            &mut self,
            _projectId: u64,
            _holder:AccountId,
            _amount:u128,
        ) -> bool {
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            if _token == AccountId::default() {
                return false
            }
            assert!(_token != AccountId::default(),"TOKEN_NOT_FOUND");
            let _unclaimedBalance = self.unclaimedBalanceOf.get(&(_holder,_projectId)).unwrap_or(&0).clone();
            let totalSupply = self.unclaimedTotalSupplyOf.get(&_projectId).unwrap_or(&0).clone();
            assert!(_unclaimedBalance >= _amount,"INSUFFICIENT_UNCLAIMED_TOKENS");
            self.unclaimedBalanceOf.insert((_holder,_projectId),_unclaimedBalance - _amount);
            self.unclaimedTotalSupplyOf.insert(_projectId,totalSupply - _amount);

            let mut token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(_token);
            token_instance.mint_token_by_owner(_holder,_amount);
            true
        }
        /**
        @notice
        Burns a project's tokens.
        @param _holder The address that owns the tokens being burned.
        @param _projectId The ID of the project to which the burned tokens belong.
        @param _amount The amount of tokens to burn.
        @param _preferClaimedTokens A flag indicating whether there's a preference for tokens to burned from the `_holder`s wallet if the project currently has a token contract attached.
      */
        #[ink(message)]
        pub fn burnFrom(
            &mut self,
           _holder:AccountId,
           _projectId:u64,
           _amount:u128,
           _preferClaimedTokens:bool
        )->bool{
            let _token = self.tokenOf.get(&_projectId).unwrap_or(&AccountId::default()).clone();
            if _token == AccountId::default() {
                return false
            }
            let mut token_instance: MBToken = ink_env::call::FromAccountId::from_account_id(_token);
            let mut _unclaimedBalance = self.unclaimedBalanceOf.get(&(_holder,_projectId)).unwrap_or(&0).clone();
            let mut _unclaimedTotalSupplyOf =   self.unclaimedTotalSupplyOf.get(&_projectId).unwrap_or(&0).clone();
            let _claimedBalance = if _token == AccountId::default() { 0 } else { token_instance.balance_of(_holder) } ;
            let mut _claimedTokensToBurn = 0;
            if _claimedBalance == 0 {
                _claimedTokensToBurn = 0;
            }else if _preferClaimedTokens {
                _claimedTokensToBurn = if _claimedBalance < _amount { _claimedBalance} else {_amount};
            } else {
                _claimedTokensToBurn = if _unclaimedBalance < _amount {_amount - _unclaimedBalance } else {0};
            }
            let _unclaimedTokensToBurn = _amount - _claimedTokensToBurn;

            if _unclaimedTokensToBurn > 0 {
                _unclaimedBalance -= _unclaimedTokensToBurn;
                _unclaimedTotalSupplyOf-=_unclaimedTokensToBurn;
                self.unclaimedBalanceOf.insert((_holder,_projectId),_unclaimedBalance);
                self.unclaimedTotalSupplyOf.insert(_projectId,_unclaimedTotalSupplyOf);
            }
            if _claimedTokensToBurn > 0 {token_instance.burn( _holder, _claimedTokensToBurn);}
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
        fn totalSupplyOf_works() {
            let  ms = MBTokenStore::new();
            assert!(ms.totalSupplyOf(1) == 0);
        }
        #[ink::test]
        fn balanceOf_works() {
            let  ms = MBTokenStore::new();
            assert!(ms.balanceOf(AccountId::default(),1) == 0);
        }
        #[ink::test]
        fn mintFor_works() {
            let mut ms = MBTokenStore::new();
            assert!(ms.mintFor(AccountId::default(),1,1,false) == false);
        }
        #[ink::test]
        fn claimFor_works() {
            let mut ms = MBTokenStore::new();
            assert!(ms.claimFor(1,AccountId::default(),1) == false);
        }

        #[ink::test]
        fn burnFrom_works() {
            let mut ms = MBTokenStore::new();
            assert!(ms.burnFrom(AccountId::default(),1,1,false) == false);
        }


    }
}
