#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;

#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod MBSplitsStore {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
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
      @member preferClaimed A flag that only has effect if a projectId is also specified, and the project has a token contract attached. If so, this flag indicates if the tokens that result from making a payment to the project should be delivered claimed into the beneficiary's wallet, or unclaimed to save gas.
      @member preferAddToBalance A flag indicating if a distribution to a project should prefer triggering it's addToBalance function instead of its pay function.
      @member percent The percent of the whole group that this split occupies.
      @member projectId The ID of a project. If an allocator is not set but a projectId is set, funds will be sent to the protocol treasury belonging to the project who's ID is specified. Resulting tokens will be routed to the beneficiary with the claimed token preference respected.
      @member beneficiary An address. The role the of the beneficary depends on whether or not projectId is specified, and whether or not an allocator is specified. If allocator is set, the beneficiary will be forwarded to the allocator for it to use. If allocator is not set but projectId is set, the beneficiary is the address to which the project's tokens will be sent that result from a payment to it. If neither allocator or projectId are set, the beneficiary is where the funds from the split will be sent.
      @member lockedUntil Specifies if the split should be unchangeable until the specified time, with the exception of extending the locked period.
      @member allocator If an allocator is specified, funds will be sent to the allocator contract along with all properties of this split.
    */
    pub struct MBSplit {
        preferClaimed:bool,
        preferAddToBalance:bool,
        percent:u64,
        projectId:u64,
        beneficiary:AccountId,
        lockedUntil:u64,
        allocator:AccountId
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
        group:u64,
        splits:Vec<MBSplit>
    }

    #[ink(storage)]
    pub struct MBSplitsStore {
        /// The number of splits currently set for each project ID's configurations.
        _splitCountOf:StorageHashMap<(u64, u64), u64>,
        ///  Packed data of splits for each project ID's configurations.
        _packedSplitParts1Of:StorageHashMap<(u64, u64, u64), u64>,
        ///  More packed data of splits for each project ID's configurations.
        _packedSplitParts2Of:StorageHashMap<(u64, u64, u64), u64>,
        ///  Mints ERC-721's that represent project ownership and transfers.
        projects : AccountId,
        /// The directory of terminals and controllers for projects.
        directory : AccountId,

    }

    impl MBSplitsStore {
        #[ink(constructor)]
        pub fn new(
            _projects:AccountId
        ) -> Self {
            Self {
                _splitCountOf: Default::default(),
                _packedSplitParts1Of: Default::default(),
                _packedSplitParts2Of: Default::default(),
                projects: _projects,
                directory: Default::default(),
            }
        }
        /**
         @notice
         Get all splits for the specified project ID, within the specified domain, for the specified group.

         @param _projectId The ID of the project to get splits for.
         @param _group The identifying group of the splits.

         @return An array of all splits for the project.
        */
        #[ink(message)]
        pub fn splitsOf(
            &self,
            _projectId: u64,
            _group: u64
        ) -> Vec<MBSplit> {
            return self._getStructsFor(_projectId, _group);
        }
        /**
            @notice
            Sets a project's splits.
            @dev
            The new splits must include any currently set splits that are locked.
            @param _projectId The ID of the project for which splits are being added.
            @param _groupedSplits An array of splits to set for any number of groups.
          */
        #[ink(message)]
        pub fn set(
            &mut self,
            _projectId: u64,
            _groupedSplits:Vec<MBGroupedSplits>
        ) ->bool {
            let _groupedSplitsLength = _groupedSplits.len();
            for i in 0.._groupedSplitsLength {
                let _groupedSplit = &_groupedSplits[i];
                let vec = _groupedSplit.splits.clone();
                self._set(_projectId, _groupedSplit.group, vec);
            }
            true
        }

        fn _set(
            &mut self,
            _projectId: u64,
            _group: u64,
            _splits:Vec<MBSplit>
        ) {
            let mut _percentTotal = 0;
            let mut _i = 0;
            // for _i in 0.._splits.len() as u64 {
            //     assert!(_splits[_i].percent != 0);
            //     _percentTotal = _percentTotal + _splits[_i].percent;
            //     let mut _packedSplitParts1;
            //     if _splits[_i].preferClaimed {_packedSplitParts1 = 1;}
            //     if _splits[_i].preferAddToBalance{ _packedSplitParts1 |= 1 << 1;}
            //     _packedSplitParts1 |= _splits[_i].percent << 2;
            //     self._packedSplitParts1Of.insert((_projectId, _group,_i),_packedSplitParts1);
            // }
            for _split in _splits.iter() {
                assert!(_split.percent != 0);
                _percentTotal = _percentTotal + _split.percent;
                let mut _packedSplitParts1 = 0;
                if _split.preferClaimed {_packedSplitParts1 = 1;}
                if _split.preferAddToBalance{ _packedSplitParts1 |= 1 << 1;}
                _packedSplitParts1 |= _split.percent << 2;
                self._packedSplitParts1Of.insert((_projectId, _group,_i),_packedSplitParts1);
                _i+=1;
            }
            self._splitCountOf.insert((_projectId, _group),_splits.len() as u64);
        }
         fn _getStructsFor(
            &self,
            _projectId: u64,
            _group: u64
        ) -> Vec<MBSplit> {
             let _splitCount = self._splitCountOf.get(&(_projectId,_group)).unwrap_or(&0).clone();
             let mut _splits = Vec::new();
             for i in 0.._splitCount{
                let _packedSplitPart1 = self._packedSplitParts1Of.get(&(_projectId,_group,i)).unwrap_or(&0).clone();
                let mut _split = MBSplit{
                    preferClaimed:false,
                    preferAddToBalance:false,
                    percent:0,
                    projectId:0,
                    beneficiary:AccountId::default(),
                    lockedUntil:0,
                    allocator:AccountId::default()
                };
                 _split.preferClaimed = _packedSplitPart1 & 1 == 1;
                 _split.preferAddToBalance = (_packedSplitPart1 >> 1) & 1 == 1;
                 _split.percent = ((_packedSplitPart1 >> 2) as u32) as u64;
                 _split.projectId = 1;
                 _split.beneficiary = AccountId::default();
                 let _packedSplitPart2 =  self._packedSplitParts2Of.get(&(_projectId,_group,i)).unwrap_or(&0).clone();
                 if _packedSplitPart2 > 0 {
                     _split.lockedUntil = (_packedSplitPart2 as u32) as u64;
                     _split.allocator = AccountId::default();
                 }
                 _splits.push(_split);
             }
             _splits
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn set_works() {
            let mut ms = MBSplitsStore::new();
            assert!( ms.set(1,Vec::new()) == true);
        }
        #[ink::test]
        fn splitsOf_works() {
            let  _split = MBSplit{
                preferClaimed:false,
                preferAddToBalance:false,
                percent:1,
                projectId:1,
                beneficiary:AccountId::default(),
                lockedUntil:0,
                allocator:AccountId::default()
            };

            let mut ms = MBSplitsStore::new();

            let mut vec = Vec::new();
            vec.push(_split);
            let mut setVec = Vec::new();
            let retVec = vec.clone();
            let _groupedSplits = MBGroupedSplits{
                group:0,
                splits:vec
            };
            setVec.push(_groupedSplits);
            ms.set(1,setVec);
            let ret = ms.splitsOf(1,0);
            assert!(ret[0].projectId == retVec[0].projectId);
        }
    }
}
