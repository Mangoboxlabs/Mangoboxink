#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
pub use self::mb_fundingCycleStore::{
    MBFundingCycleStore
};
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod mb_fundingCycleStore {
    use alloc::string::String;
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
    /**
      @member duration The number of seconds the funding cycle lasts for, after which a new funding cycle will start. A duration of 0 means that the funding cycle will stay active until the project owner explicitly issues a reconfiguration, at which point a new funding cycle will immediately start with the updated properties. If the duration is greater than 0, a project owner cannot make changes to a funding cycle's parameters while it is active – any proposed changes will apply to the subsequent cycle. If no changes are proposed, a funding cycle rolls over to another one with the same properties but new `start` timestamp and a discounted `weight`.
      @member weight A fixed point number with 18 decimals that contracts can use to base arbitrary calculations on. For example, payment terminals can use this to determine how many tokens should be minted when a payment is received.
      @member discountRate A percent by how much the `weight` of the subsequent funding cycle should be reduced, if the project owner hasn't configured the subsequent funding cycle with an explicit `weight`. If it's 0, each funding cycle will have equal weight. If the number is 90%, the next funding cycle will have a 10% smaller weight. This weight is out of `JBConstants.MAX_DISCOUNT_RATE`.
      @member ballot An address of a contract that says whether a proposed reconfiguration should be accepted or rejected. It can be used to create rules around how a project owner can change funding cycle parameters over time.
    */
    pub struct MBFundingCycleData {
        duration: u64,
        weight: u64,
        discountRate: u64,
        ballot: AccountId,
    }

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
        number:u64,
        configuration:u64,
        basedOn:u64,
        start:u64,
        duration:u64,
        weight:u64,
        discountRate:u64,
        ballot:AccountId,
        metadata:u64
    }
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MBBallotState {
        Active,
        Approved,
        Failed
    }
    #[ink(storage)]
    pub struct MBFundingCycleStore {
        ///  Stores the user defined properties of each funding cycle, packed into one storage slot.
        _packedUserPropertiesOf:StorageHashMap<(u64, u64), u64>,
        ///  Stores the metadata for each funding cycle configuration, packed into one storage slot.
        _metadataOf:StorageHashMap<(u64, u64), u64>,
        ///  Stores the properties added by the mechanism to manage and schedule each funding cycle, packed into one storage slot.
        _packedIntrinsicPropertiesOf:StorageHashMap<(u64, u64), u128>,
        ///The ID of the project that each token belongs to.
        latestConfigurationOf:StorageHashMap<u64,u64>,
        projectsWeight:StorageHashMap<u64,u128>,
    }

    impl MBFundingCycleStore {
        #[ink(constructor)]
        pub fn new(
        ) -> Self {
            Self {
                _packedUserPropertiesOf:Default::default(),
                _metadataOf:Default::default(),
                _packedIntrinsicPropertiesOf:Default::default(),
                latestConfigurationOf:Default::default(),
                projectsWeight:Default::default()
            }
        }
                /**
            @notice
            Get the funding cycle with the given configuration for the specified project.

            @param _projectId The ID of the project to which the funding cycle belongs.
            @param _configuration The configuration of the funding cycle to get.

            @return fundingCycle The funding cycle.
          */
        #[ink(message)]
        pub fn get(
            &self,
            _projectId: u64,
            _configuration: u64,
        ) -> MBFundingCycle {
            return self._getStructFor(_projectId, _configuration);
        }
        /**
      @notice
      The latest funding cycle to be configured for the specified project, and its current ballot state.

      @param _projectId The ID of the project to get the latest configured funding cycle of.

      @return fundingCycle The project's queued funding cycle.
      @return ballotState The state of the ballot for the reconfiguration.
    */
        #[ink(message)]
        pub fn latestConfiguredOf(
            &self,
            _projectId: u64
        ) -> (MBFundingCycle,MBBallotState) {
            let _fundingCycleConfiguration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            let fundingCycle = self._getStructFor(_projectId, _fundingCycleConfiguration);
            let ballotState = self._ballotStateOf(
                _projectId,
                fundingCycle.configuration,
                fundingCycle.start,
                fundingCycle.basedOn
            );
            return (fundingCycle,ballotState);
        }
        /**
          @notice
          The funding cycle that's next up for the specified project.

          @dev
          If a queued funding cycle of the project is not found, returns an empty funding cycle with all properties set to 0.

          @param _projectId The ID of the project to get the queued funding cycle of.

          @return fundingCycle The project's queued funding cycle.
        */
        #[ink(message)]
        pub fn queuedOf(
            &self,
            _projectId: u64
        ) -> MBFundingCycle {
            let _fundingCycleConfiguration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            if _fundingCycleConfiguration == 0 {
                return self._getStructFor(0, 0);
            }

            let _standbyFundingCycleConfiguration = self._standbyOf(_projectId);
            if _standbyFundingCycleConfiguration > 0 {
                let fundingCycle = self._getStructFor(_projectId, _standbyFundingCycleConfiguration);

                if self._isApproved(_projectId, &fundingCycle){
                    return fundingCycle;
                }
                if fundingCycle.duration == 0{
                    return self._getStructFor(0, 0);
                }
            } else {
                let mut fundingCycle = self._getStructFor(_projectId, _fundingCycleConfiguration);
                if fundingCycle.start > self.env().block_timestamp(){
                    fundingCycle = self._getStructFor(_projectId, 0);
                }
                if fundingCycle.duration == 0{
                    return self._getStructFor(0, 0);
                }
            }
            return self._getStructFor(0, 0);
        }
        /**
            @notice
            The funding cycle that is currently active for the specified project.

            @dev
            If a current funding cycle of the project is not found, returns an empty funding cycle with all properties set to 0.

            @param _projectId The ID of the project to get the current funding cycle of.

            @return fundingCycle The project's current funding cycle.
          */
        #[ink(message)]
        pub fn currentOf(
            &self,
            _projectId:u64,
        ) -> MBFundingCycle {
            let mut _fundingCycleConfiguration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            if _fundingCycleConfiguration == 0 {
                return self._getStructFor(0, 0);
            }

            if _fundingCycleConfiguration > 0 {
                let mut _fundingCycle = self._getStructFor(_projectId, _fundingCycleConfiguration);
                if self._isApproved(_projectId, &_fundingCycle) {
                    return _fundingCycle;
                }
                _fundingCycleConfiguration = _fundingCycle.basedOn;
                return _fundingCycle;
            } else {
                let mut _fundingCycle = self._getStructFor(_projectId, _fundingCycleConfiguration);
                if !self._isApproved(_projectId, &_fundingCycle) || self.env().block_timestamp() < _fundingCycle.start {
                    _fundingCycleConfiguration = _fundingCycle.basedOn;
                }
                return _fundingCycle;
            }
            // if _fundingCycleConfiguration == 0 {
            //     return self._getStructFor(0, 0);
            // }

        }
        /**
         @notice
         GetProjectsWeight
         @param _projectId The ID of the project to check the ballot state of.
       */
        #[ink(message)]
        pub fn getProjectsWeight(
            &self,
            _projectId:u64,
        ) ->u128 {
            self.projectsWeight.get(&_projectId).unwrap_or(&0).clone()
        }

        /**
         @notice
         Get 1 USD to token amount
         @param _projectId The ID of the project to check the ballot state of.
       */
        #[ink(message)]
        pub fn getChangeAmount(
            &self,
            _projectId:u64,
        ) ->u128 {
            return self.getProjectsWeight(_projectId) / 1000000000000000000
        }
        /**
           @notice
           The current ballot state of the project.

           @param _projectId The ID of the project to check the ballot state of.

           @return The project's current ballot's state.
         */
        #[ink(message)]
        pub fn currentBallotStateOf(
            &self,
            _projectId:u64,
        ) -> MBBallotState {
            let _fundingCycleConfiguration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            let _fundingCycle = self._getStructFor(_projectId, _fundingCycleConfiguration);
            return
               self._ballotStateOf(
                    _projectId,
                    _fundingCycle.configuration,
                    _fundingCycle.start,
                    _fundingCycle.basedOn
                );
        }
        /**
           @notice
           Configures the next eligible funding cycle for the specified project.
           @param _projectId The ID of the project being configured.
           @param _data The funding cycle configuration data.
           @param _metadata Arbitrary extra data to associate with this funding cycle configuration that's not used within.
           @param _mustStartAtOrAfter The time before which the initialized funding cycle cannot start.

           @return The funding cycle that the configuration will take effect during.
         */
        #[ink(message)]
        pub fn configureFor(
            &mut self,
            _projectId:u64,
            _weight:u128,
            _metadata:u64,
            _mustStartAtOrAfter:u64
        ) -> MBFundingCycle {
            let _configuration = self.env().block_timestamp();
            self.projectsWeight.insert(_projectId,_weight);
            self._configureIntrinsicPropertiesFor(
                _projectId,
                _configuration,
                _weight,
               if  _mustStartAtOrAfter > self.env().block_timestamp() {_mustStartAtOrAfter } else {self.env().block_timestamp()}
            );
            return self._getStructFor(_projectId, _configuration);
        }

        fn _configureIntrinsicPropertiesFor(
            &mut self,
            _projectId:u64,
            _configuration:u64,
            _weight:u128,
            _mustStartAtOrAfter:u64
        ){
            let _fundingCycleConfiguration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            self._initFor(_projectId, _configuration,_weight, _mustStartAtOrAfter,self._getStructFor(0, 0));
        }

        fn _initFor(
            &mut self,
            _projectId:u64,
            _configuration:u64,
            _weight:u128,
            _mustStartAtOrAfter:u64,
            _baseFundingCycle:MBFundingCycle
        ){
            let _number = 1;
            self._packAndStoreIntrinsicPropertiesOf(
                _configuration,
                _projectId,
                _number,
                _weight,
                _baseFundingCycle.configuration,
                _mustStartAtOrAfter
            );
            self.latestConfigurationOf.insert(_projectId,_configuration);
        }
        fn _packAndStoreIntrinsicPropertiesOf(
            &mut self,
            _projectId:u64,
            _configuration:u64,
            _weight:u128,
            _number:u128,
            _basedOn:u64,
            _start:u64
        ){
            let mut packed = _weight;
            packed |= _number << 20;
            self._packedIntrinsicPropertiesOf.insert((_projectId,_configuration),packed);
        }

        fn _isApproved(
            &self,
            _projectId:u64,
            _fundingCycle:&MBFundingCycle
        ) -> bool {
            return self._ballotStateOf(
                _projectId,
                _fundingCycle.configuration,
                _fundingCycle.start,
                _fundingCycle.basedOn
            ) == MBBallotState::Approved;
        }
        fn _standbyOf(
            &self,
            _projectId:u64,
        ) -> u64 {
            let configuration = self.latestConfigurationOf.get(&_projectId).unwrap_or(&0).clone();
            let _fundingCycle = self._getStructFor(_projectId, configuration);
            if self.env().block_timestamp() > _fundingCycle.start{
                return 0;
            }
            if _fundingCycle.number == 1 {
                return configuration;
            }
            let _baseFundingCycle = self._getStructFor(_projectId, _fundingCycle.basedOn);
            if _baseFundingCycle.duration > 0 && self.env().block_timestamp() < _fundingCycle.start - _baseFundingCycle.duration {
                return 0;
            }
            return 0;
        }
        fn _ballotStateOf(
            &self,
            _projectId:u64,
            _configuration:u64,
            _start:u64,
            _ballotFundingCycleConfiguration:u64
        ) -> MBBallotState {
            if _ballotFundingCycleConfiguration == 0 {
                return MBBallotState::Approved;
            }
            let _ballotFundingCycle = self._getStructFor(_projectId, _ballotFundingCycleConfiguration);
            if _ballotFundingCycle.ballot == AccountId::default() {
                return MBBallotState::Approved;
            } else {
                return  MBBallotState::Active;
            }
        }
        fn _getStructFor(
            &self,
            _projectId: u64,
            _configuration: u64
        )-> MBFundingCycle {
            let mut fundingCycle = MBFundingCycle{
                number:0,
                configuration:0,
                basedOn:0,
                start:0,
                duration:0,
                weight:0,
                discountRate:0,
                ballot:AccountId::default(),
                metadata:0
            };
            if _configuration == 0 {
                return fundingCycle;
            }
            fundingCycle.configuration = _configuration;
            let _packedIntrinsicProperties = self._packedIntrinsicPropertiesOf.get(&(_projectId,_configuration)).unwrap_or(&0).clone();
            let _packedUserPropertiesOf = self._packedUserPropertiesOf.get(&(_projectId,_configuration)).unwrap_or(&0).clone();
            fundingCycle.weight = (_packedIntrinsicProperties as u128) as u64;
            fundingCycle.basedOn = ((_packedIntrinsicProperties >> 8) as u128) as u64;
            fundingCycle.start  = ((_packedIntrinsicProperties >> 14) as u128) as u64;
            fundingCycle.number  = ((_packedIntrinsicProperties >> 20) as u128) as u64;
            fundingCycle.ballot  = AccountId::default();
            fundingCycle.duration = ((_packedUserPropertiesOf >> 16) as u128) as u64;
            fundingCycle.discountRate = ((_packedUserPropertiesOf >> 22) as u128) as u64;
            fundingCycle.metadata = self._metadataOf.get(&(_projectId,_configuration)).unwrap_or(&0).clone();
            fundingCycle
        }


    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn configureFor_works() {
            let mut ms = MBFundingCycleStore::new();
            let fundingCycle = ms.configureFor(0,0,0,0);
            assert!(fundingCycle.number == 0);
        }
        #[ink::test]
        fn currentBallotStateOf_works() {
            let ms = MBFundingCycleStore::new();
            let ret = ms.currentBallotStateOf(1);
            assert!(ret == MBBallotState::Approved);
        }
        #[ink::test]
        fn currentOf_works() {
            let ms = MBFundingCycleStore::new();
            let ret = ms.currentOf(1);
            assert!(ret.number ==0);
        }
        #[ink::test]
        fn queuedOf_works() {
            let ms = MBFundingCycleStore::new();
            let ret = ms.queuedOf(1);
            assert!(ret.number ==0);
        }
        #[ink::test]
        fn latestConfiguredOf_works() {
            let ms = MBFundingCycleStore::new();
            let (ret,res) = ms.latestConfiguredOf(1);
            assert!(ret.number == 0);
            assert!(res == MBBallotState::Approved);
        }
        #[ink::test]
        fn get_works() {
            let ms = MBFundingCycleStore::new();
            let ret = ms.get(1,1);
            assert!(ret.number == 0);
        }
    }
}
