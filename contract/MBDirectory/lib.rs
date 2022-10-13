#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use ink_lang as ink;
pub use self::mbDirectory::{
    MBDirectory
};
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[ink::contract]
mod mbDirectory {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };


    #[ink(storage)]
    pub struct MBDirectory {
        projects: AccountId,
        ///For each project ID, the terminals that are currently managing its funds.
        _terminalsOf: StorageHashMap<u64, Vec<AccountId>>,
        /// The project's primary terminal for a token.
        _primaryTerminalOf: StorageHashMap<(u64, AccountId), AccountId>,
        /// The contract storing all funding cycle configurations.
        fundingCycleStore: AccountId,
        ///    For each project ID, the controller that manages how terminals interact with tokens and funding cycles.
        controllerOf: StorageHashMap<u64, AccountId>,
        ///    Addresses that can set a project's first controller on their behalf. These addresses/contracts have been vetted and verified by this contract's owner.
        isAllowedToSetFirstController: StorageHashMap<AccountId, bool>,
    }

    impl MBDirectory {
        #[ink(constructor)]
        pub fn new(
            _projects:AccountId,
            _fundingCycleStore:AccountId,
        ) -> Self {
            Self {
                projects: _projects,
                _terminalsOf: Default::default(),
                _primaryTerminalOf: Default::default(),
                fundingCycleStore:_fundingCycleStore,
                controllerOf: Default::default(),
                isAllowedToSetFirstController: Default::default(),
            }
        }
        /**
           @notice
           For each project ID, the terminals that are currently managing its funds.

           @param _projectId The ID of the project to get terminals of.

           @return An array of terminal addresses.
         */
        #[ink(message)]
        pub fn terminalsOf(
            &self,
            _projectId: u64,
        ) -> Vec<AccountId> {
            let defaultVec = Vec::new();
            self._terminalsOf.get(&_projectId).unwrap_or(&defaultVec).clone()
        }
        /**
        @notice
        The primary terminal that is managing funds for a project for a specified token.

        @dev
        The zero address is returned if a terminal isn't found for the specified token.

        @param _projectId The ID of the project to get a terminal for.
        @param _token The token the terminal accepts.

        @return The primary terminal for the project for the specified token.
      */
        #[ink(message)]
        pub fn primaryTerminalOf(
            &self,
            _token: AccountId,
            _projectId: u64,
        ) -> AccountId {
            let terminal = self._primaryTerminalOf.get(&(_projectId, _token)).unwrap_or(&AccountId::default()).clone();
            let isTerminal = self.isTerminalOf(_projectId, terminal);
            if terminal != AccountId::default() && isTerminal {
                return terminal;
            }
            return AccountId::default();
        }
            /**
            @notice
            Whether or not a specified terminal is a terminal of the specified project.

            @param _projectId The ID of the project to check within.
            @param _terminal The address of the terminal to check for.

            @return A flag indicating whether or not the specified terminal is a terminal of the specified project.
          */
        #[ink(message)]
        pub fn isTerminalOf(
            &self,
            _projectId: u64,
            _terminal: AccountId
        ) -> bool {
                let defaultVec = Vec::new();
               let vec =  self._terminalsOf.get(&_projectId).unwrap_or(&defaultVec).clone();
                for _i in vec.iter() {
                    if _i ==  &_terminal{
                        return true;
                    }
                }
                return false;
        }
            /**
         @notice
         Update the controller that manages how terminals interact with the ecosystem.

         @param _projectId The ID of the project to set a new controller for.
         @param _controller The new controller to set.
       */
        #[ink(message)]
        pub fn setControllerOf(
            &mut self,
            _projectId: u64,
            _controller: AccountId,
        ) -> bool {
            self.controllerOf.insert(_projectId,_controller);
            true
        }
        /**
        @notice
        Set a project's terminals.

        @param _projectId The ID of the project having terminals set.
        @param _terminals The terminal to set.
      */
        #[ink(message)]
        pub fn setTerminalsOf(
            &mut self,
            _projectId: u64,
            _terminals: Vec<AccountId>,
        ) -> bool {
            self._terminalsOf.insert(_projectId,_terminals);
            true
        }
        /**
        @notice
        Project's can set which terminal should be their primary for a particular token.
        This is useful in case a project has several terminals connected for a particular token.

        @param _projectId The ID of the project for which a primary token is being set.
        @param _token The token to set the primary terminal of.
        @param _terminal The terminal to make primary.
      */
        #[ink(message)]
        pub fn setPrimaryTerminalOf(
            &mut self,
            _projectId: u64,
            _token: AccountId,
            _terminal: AccountId,
        ) -> bool {
            self._addTerminalIfNeeded(_projectId, _terminal);
            self._primaryTerminalOf.insert((_projectId,_token),_terminal);
            true
        }

        fn _addTerminalIfNeeded(
            &mut self,
            _projectId: u64,
            _terminal:AccountId
        ){
            if self.isTerminalOf(_projectId, _terminal) {return;}
            let defaultVec = Vec::new();
            let mut vec =  self._terminalsOf.get(&_projectId).unwrap_or(&defaultVec).clone();
            vec.push(_terminal);
            self._terminalsOf.insert(_projectId,vec);
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;
        #[ink::test]
        fn setTerminalsOf_works() {
            let mut directory = MBDirectory::new();
            let mut vec = Vec::new();
            vec.push(AccountId::default());
            assert!(directory.setTerminalsOf(1,vec) == true);
        }
        #[ink::test]
        fn setPrimaryTerminalOf_works() {
            let mut directory = MBDirectory::new();
            assert!(directory.setPrimaryTerminalOf(1,AccountId::default(),AccountId::default()) == true);
        }
        #[ink::test]
        fn setControllerOf_works() {
            let mut directory = MBDirectory::new();
            assert!(directory.setControllerOf(1,AccountId::default()) == true);
        }
        #[ink::test]
        fn isTerminalOf_works() {
            let directory = MBDirectory::new();
            assert!(directory.isTerminalOf(1,AccountId::default()) == false);
        }
        #[ink::test]
        fn primaryTerminalOf_works() {
            let directory = MBDirectory::new();
            assert!(directory.primaryTerminalOf(AccountId::default(),1) == AccountId::default());
        }
        #[ink::test]
        fn terminalsOf_works() {
            let mut directory = MBDirectory::new();
            let mut vec = Vec::new();
            vec.push(AccountId::default());
            let newVec = vec.clone();
            directory.setTerminalsOf(1,vec);
            assert!(directory.terminalsOf(1)[0] == newVec[0]);
        }
    }
}
