#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;

#[allow(unused_imports)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[ink::contract]
mod mb_controller {
    use mb_projects::MBProjects;
    use mbdirectory::MBDirectory;
    use mbtokenstore::MBTokenStore;
    use mb_fundingcyclestore::MBFundingCycleStore;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    ///content The metadata content.
    ///domain The domain within which the metadata applies.
    pub struct MBProjectMetadata {
        content: String,
        domain: u64,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member duration The number of seconds the funding cycle lasts for, after which a new funding cycle will start. A duration of 0 means that the funding cycle will stay active until the project owner explicitly issues a reconfiguration, at which point a new funding cycle will immediately start with the updated properties. If the duration is greater than 0, a project owner cannot make changes to a funding cycle's parameters while it is active – any proposed changes will apply to the subsequent cycle. If no changes are proposed, a funding cycle rolls over to another one with the same properties but new `start` timestamp and a discounted `weight`.
      @member weight A fixed point number with 18 decimals that contracts can use to base arbitrary calculations on. For example, payment terminals can use this to determine how many tokens should be minted when a payment is received.
      @member discountRate A percent by how much the `weight` of the subsequent funding cycle should be reduced, if the project owner hasn't configured the subsequent funding cycle with an explicit `weight`. If it's 0, each funding cycle will have equal weight. If the number is 90%, the next funding cycle will have a 10% smaller weight. This weight is out of `JBConstants.MAX_DISCOUNT_RATE`.
      @member ballot An address of a contract that says whether a proposed reconfiguration should be accepted or rejected. It can be used to create rules around how a project owner can change funding cycle parameters over time.
    */
    pub struct MBFundingCycleData {
        duration: u64,
        weight: u128,
        discountRate: u64,
        ballot: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
    @member global Data used globally in non-migratable ecosystem contracts.
    @member reservedRate The reserved rate of the funding cycle.
    @member redemptionRate The redemption rate of the funding cycle.
    @member ballotRedemptionRate The redemption rate to use during an active ballot of the funding cycle.
    @member pausePay A flag indicating if the pay functionality should be paused during the funding cycle.
    @member pauseDistributions A flag indicating if the distribute functionality should be paused during the funding cycle.
    @member pauseRedeem A flag indicating if the redeem functionality should be paused during the funding cycle.
    @member pauseBurn A flag indicating if the burn functionality should be paused during the funding cycle.
    @member allowMinting A flag indicating if the mint functionality should be allowed during the funding cycle.
    @member allowChangeToken A flag indicating if changing tokens should be allowed during this funding cycle.
    @member allowTerminalMigration A flag indicating if migrating terminals should be allowed during this funding cycle.
    @member allowControllerMigration A flag indicating if migrating controllers should be allowed during this funding cycle.
    @member holdFees A flag indicating if fees should be held during this funding cycle.
    @member useTotalOverflowForRedemptions A flag indicating if redemptions should use the project's balance held in all terminals instead of the project's local terminal balance from which the redemption is being fulfilled.
    @member useDataSourceForPay A flag indicating if the data source should be used for pay transactions during this funding cycle.
    @member useDataSourceForRedeem A flag indicating if the data source should be used for redeem transactions during this funding cycle.
    @member dataSource The data source to use during this funding cycle.
  */
    pub struct MBFundingCycleMetadata {
        global: AccountId,
        reservedRate: u64,
        redemptionRate: u64,
        ballotRedemptionRate: u64,
        pausePay: bool,
        pauseDistributions: bool,
        pauseRedeem: bool,
        pauseBurn: bool,
        allowMinting: bool,
        allowChangeToken: bool,
        allowTerminalMigration: bool,
        allowControllerMigration: bool,
        holdFees: bool,
        useTotalOverflowForRedemptions: bool,
        useDataSourceForPay: bool,
        useDataSourceForRedeem: bool,
        dataSource: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
       @member group The group indentifier.
       @member splits The splits to associate with the group.
    */
    pub struct MBGroupedSplits {
        group: u64,
        splits: BTreeMap<u64, MBSplit>,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member preferClaimed A flag that only has effect if a projectId is also specified, and the project has a token contract attached. If so, this flag indicates if the tokens that result from making a payment to the project should be delivered claimed into the beneficiary's wallet, or unclaimed to save gas.
      @member preferAddToBalance A flag indicating if a distribution to a project should prefer triggering it's addToBalance function instead of its pay function.
      @member percent The percent of the whole group that this split occupies. This number is out of `JBConstants.SPLITS_TOTAL_PERCENT`.
      @member projectId The ID of a project. If an allocator is not set but a projectId is set, funds will be sent to the protocol treasury belonging to the project who's ID is specified. Resulting tokens will be routed to the beneficiary with the claimed token preference respected.
      @member beneficiary An address. The role the of the beneficary depends on whether or not projectId is specified, and whether or not an allocator is specified. If allocator is set, the beneficiary will be forwarded to the allocator for it to use. If allocator is not set but projectId is set, the beneficiary is the address to which the project's tokens will be sent that result from a payment to it. If neither allocator or projectId are set, the beneficiary is where the funds from the split will be sent.
      @member lockedUntil Specifies if the split should be unchangeable until the specified time, with the exception of extending the locked period.
      @member allocator If an allocator is specified, funds will be sent to the allocator contract along with all properties of this split.
    */
    pub struct MBSplit {
        preferClaimed: bool,
        preferAddToBalance: bool,
        percent: u64,
        projectId: u64,
        beneficiary: AccountId,
        lockedUntil: u64,
        allocator: AccountId,
    }

    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    /**
      @member terminal The terminal within which the distribution limit and the overflow allowance applies.
      @member token The token for which the fund access constraints apply.
      @member distributionLimit The amount of the distribution limit, as a fixed point number with the same number of decimals as the terminal within which the limit applies.
      @member distributionLimitCurrency The currency of the distribution limit.
      @member overflowAllowance The amount of the allowance, as a fixed point number with the same number of decimals as the terminal within which the allowance applies.
      @member overflowAllowanceCurrency The currency of the overflow allowance.
    */
    pub struct MBFundAccessConstraints {
        terminal: AccountId,
        token: AccountId,
        distributionLimit: u64,
        distributionLimitCurrency: u64,
        overflowAllowance: u64,
        overflowAllowanceCurrency: u64,
    }

    /**
     _operatorStore A contract storing operator assignments.
     _projects A contract which mints ERC-721's that represent project ownership and transfers.
     _directory A contract storing directories of terminals and controllers for each project.
     _fundingCycleStore A contract storing all funding cycle configurations.
     _tokenStore A contract that manages token minting and burning.
     _splitsStore A contract that stores splits for each project.
     _processedTokenTrackerOf The difference between the processed token tracker of a project and the project's token's total supply is the amount of tokens that still need to have reserves minted against them.

  */
    #[ink(storage)]
    pub struct MBController {
        operatorStore: AccountId,
        projects: AccountId,
        directory: AccountId,
        fundingCycleStore: AccountId,
        tokenStore: AccountId,
        splitsStore: AccountId,
        _processedTokenTrackerOf: StorageHashMap<u64, u128>,
        ownerProjects:StorageHashMap<AccountId, Vec<u64>>
    }

    impl MBController {
        #[ink(constructor)]
        pub fn new(
            _operatorStore: AccountId,
            _projects: AccountId,
            _directory: AccountId,
            _fundingCycleStore: AccountId,
            _tokenStore: AccountId,
            _splitsStore: AccountId,
        ) -> Self {
            Self {
                operatorStore: _operatorStore,
                projects: _projects,
                directory: _directory,
                fundingCycleStore: _fundingCycleStore,
                tokenStore: _tokenStore,
                splitsStore: _splitsStore,
                _processedTokenTrackerOf:Default::default(),
                ownerProjects:Default::default()
            }
        }

        /**
           @notice
           Creates a project. This will mint an ERC-721 into the specified owner's account, configure a first funding cycle, and set up any splits.

           @param _owner The address to set as the owner of the project. The project ERC-721 will be owned by this address.
           @param _projectMetadata Metadata to associate with the project within a particular domain. This can be updated any time by the owner of the project.
           @param _data Data that defines the project's first funding cycle. These properties will remain fixed for the duration of the funding cycle.
           @param _metadata Metadata specifying the controller specific params that a funding cycle can have. These properties will remain fixed for the duration of the funding cycle.
           @param _mustStartAtOrAfter The time before which the configured funding cycle cannot start.
           @param _groupedSplits An array of splits to set for any number of groups.
           @param _fundAccessConstraints An array containing amounts that a project can use from its treasury for each payment terminal. Amounts are fixed point numbers using the same number of decimals as the accompanying terminal.
           @param _terminals Payment terminals to add for the project.
           @param _memo A memo to pass along to the emitted event.

           @return projectId The ID of the project.
         */
        #[ink(message)]
        pub fn launchProjectFor(
            &mut self,
            _owner: AccountId,
            _projectMetadata: String,
            _data: MBFundingCycleData,
            _metadata: u64,
            _mustStartAtOrAfter: u64,
            _groupedSplits: Vec<MBGroupedSplits>,
            _fundAccessConstraints: Vec<MBFundAccessConstraints>,
            _terminals: Vec<AccountId>,
            _memo: String,
        ) -> u64 {
            let mut projects: MBProjects = ink_env::call::FromAccountId::from_account_id(self.projects);
            let mut directory: MBDirectory = ink_env::call::FromAccountId::from_account_id(self.directory);
            if self.projects == AccountId::default() {
                return 0
            }
            let projectId = projects.create_for(_owner, _projectMetadata);
            directory.setControllerOf(projectId, Self::env().account_id());
            self._configure(
                projectId,
                _data,
                _metadata,
                _mustStartAtOrAfter,
                _groupedSplits,
                _fundAccessConstraints,
            );
            if _terminals.len() > 0 { let _ret = directory.setTerminalsOf(projectId, _terminals); }
            let mut ownerProjects = self.ownerProjects.get(&_owner).unwrap_or(&Vec::new()).clone();
            ownerProjects.push(projectId);
            self.ownerProjects.insert(_owner,ownerProjects);
            return projectId;
        }


        /**
        @notice
        Get user's projects
        @param _owner The address to set as the owner of the project. The project ERC-721 will be owned by this address.
      */
        #[ink(message)]
        pub fn getOwnerProjects(&self, owner: AccountId) -> Vec<u64> {
            self.ownerProjects.get(&owner).unwrap_or(&Vec::new()).clone()
        }
        /**
          @notice
          Issues an owner's ERC20 JBTokens that'll be used when claiming tokens.

          @param _projectId The ID of the project being issued tokens.
          @param _name The ERC20's name.
          @param _symbol The ERC20's symbol.
        */
        #[ink(message)]
        pub fn issueTokenFor(
            &mut self,
            _projectId: u64,
            _name: String,
            _symbol: String,
        ) -> AccountId {
            let mut tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
            if self.tokenStore == AccountId::default() {
                return AccountId::default()
            }
            return tokenStore.issueFor(_projectId, _name, _symbol);
        }
        /**
       @notice
       Mint new token supply into an account, and optionally reserve a supply to be distributed according to the project's current funding cycle configuration.
       @param _projectId The ID of the project to which the tokens being minted belong.
       @param _tokenCount The amount of tokens to mint in total, counting however many should be reserved.
       @param _beneficiary The account that the tokens are being minted for.
       @param _memo A memo to pass along to the emitted event.
       @param _preferClaimedTokens A flag indicating whether a project's attached token contract should be minted if they have been issued.

       @return beneficiaryTokenCount The amount of tokens minted for the beneficiary.
     */
        #[ink(message)]
        pub fn mintTokensOf(
            &mut self,
            _projectId: u64,
            _tokenCount: u128,
            _beneficiary: AccountId,
            _memo: String,
            _preferClaimedTokens: bool,
        ) -> u128 {
            let fundingCycleStore:MBFundingCycleStore = ink_env::call::FromAccountId::from_account_id(self.fundingCycleStore);
            if self.fundingCycleStore == AccountId::default() {
                return 0
            }
            let mut tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
            let _fundingCycle = fundingCycleStore.currentOf(_projectId);
            let beneficiaryTokenCount = _tokenCount;
            let mut processedTokenTrackerOf = self._processedTokenTrackerOf.get(&_projectId).unwrap_or(&0).clone();
            processedTokenTrackerOf += beneficiaryTokenCount;
            self._processedTokenTrackerOf.insert(_projectId, processedTokenTrackerOf);
            tokenStore.mintFor(_beneficiary, _projectId, beneficiaryTokenCount, _preferClaimedTokens);
            return beneficiaryTokenCount;
        }
        /**
        @notice
        Burns a token holder's supply.
        @param _holder The account that is having its tokens burned.
        @param _projectId The ID of the project to which the tokens being burned belong.
        @param _tokenCount The number of tokens to burn.
        @param _memo A memo to pass along to the emitted event.
        @param _preferClaimedTokens A flag indicating whether a project's attached token contract should be burned first if they have been issued.
      */
        pub fn burnTokensOf(
            &mut self,
            _holder: AccountId,
            _projectId: u64,
            _tokenCount: u128,
            _memo: String,
            _preferClaimedTokens: bool,
        ) ->bool {
            let fundingCycleStore:MBFundingCycleStore = ink_env::call::FromAccountId::from_account_id(self.fundingCycleStore);
            if self.fundingCycleStore == AccountId::default() {
                return false
            }
            let _fundingCycle = fundingCycleStore.currentOf(_projectId);
            let mut processedTokenTrackerOf = self._processedTokenTrackerOf.get(&_projectId).unwrap_or(&0).clone();
            processedTokenTrackerOf -= _tokenCount;
            self._processedTokenTrackerOf.insert(_projectId,processedTokenTrackerOf);
            let mut tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
            let ret = tokenStore.burnFrom(_holder, _projectId, _tokenCount, _preferClaimedTokens);
            ret
        }
        /**
          @notice
          Distributes all outstanding reserved tokens for a project.

          @param _projectId The ID of the project to which the reserved tokens belong.
          @param _memo A memo to pass along to the emitted event.

          @return The amount of minted reserved tokens.
        */
        pub fn distributeReservedTokensOf(
            &mut self,
            _projectId:u64,
            _memo:String
        )->u128{
            return self._distributeReservedTokensOf(_projectId, _memo);
        }
        fn _distributeReservedTokensOf(
            &mut self,
            _projectId:u64,
            _memo:String
        )->u128{
            let fundingCycleStore:MBFundingCycleStore = ink_env::call::FromAccountId::from_account_id(self.fundingCycleStore);
            if self.fundingCycleStore == AccountId::default() {
                return 0
            }
            let  _fundingCycle = fundingCycleStore.currentOf(_projectId);
            let  tokenStore:MBTokenStore = ink_env::call::FromAccountId::from_account_id(self.tokenStore);
            let _totalTokens = tokenStore.totalSupplyOf(_projectId);
            self._processedTokenTrackerOf.insert(_projectId,_totalTokens);
            _totalTokens
        }

        fn _configure(
            &mut self,
            _projectId: u64,
            _data: MBFundingCycleData,
            _metadata: u64,
            _mustStartAtOrAfter: u64,
            _groupedSplits: Vec<MBGroupedSplits>,
            _fundAccessConstraints: Vec<MBFundAccessConstraints>,
        ) {
            let mut fundingCycleStore:MBFundingCycleStore = ink_env::call::FromAccountId::from_account_id(self.fundingCycleStore);
            fundingCycleStore.configureFor(
                _projectId,
                _data.weight,
                _metadata,
                _mustStartAtOrAfter,
            );
            // Set splits for the group.
            //splitsStore.set(_projectId, _fundingCycle.configuration, _groupedSplits);
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn launchProjectFor_works() {
            let data =  MBFundingCycleData {
                duration: 1,
                weight: 1,
                discountRate: 1,
                ballot:  AccountId::default()
            };
            let mut mc = MBController::new(
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default()
            );
           let ret =  mc.launchProjectFor(
                AccountId::default(),
                String::from("test"),
                data,
                1,
                1,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                String::from("test")
            );
            assert!(ret == 0);
        }
        #[ink::test]
        fn issueTokenFor_works() {
            let mut mc = MBController::new(
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default()
            );
            let ret = mc.issueTokenFor(1, String::from("test"), String::from("test"));
            assert!(ret == AccountId::default());
        }
        #[ink::test]
        fn mintTokensOf_works() {
            let mut mc = MBController::new(
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default()
            );
            let ret = mc.mintTokensOf(1,1,AccountId::default(),String::from("test"),false);
            assert!(ret == 0);
        }
        #[ink::test]
        fn burnTokensOf_works() {
            let mut mc = MBController::new(
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default()
            );
            let ret = mc.burnTokensOf(AccountId::default(),1,1,String::from("test"),false);
            assert!(ret == false);
        }
        #[ink::test]
        fn distributeReservedTokensOf_works() {
            let mut mc = MBController::new(
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default(),
                AccountId::default()
            );
            let ret = mc.distributeReservedTokensOf(1,String::from("test"));
            assert!(ret == 0);
        }
    }
}
