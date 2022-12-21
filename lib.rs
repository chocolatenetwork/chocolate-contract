#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
// PackedLayout
#[ink::contract]
mod flipper {
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use ink_storage::Mapping;
    // Ref: https://github.com/paritytech/ink/blob/master/examples/mother/Cargo.toml
    use ink_prelude::vec::Vec;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default, SpreadAllocate)]
    #[ink(storage)]
    pub struct Chocolate {
        project_index: u32,
        /// Stores a project from id in storage
        projects: Mapping<u32, Project>,
        // Multivalued keys would allow wider scope for reviews
        /// Index in reviews_projects_list arr. -> struct
        reviews: Mapping<u32, Review>,
        /// Accountid + projectId
        reviews_projects_list: Vec<(AccountId, u32)>,
    }

    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
        PackedLayout,
        SpreadAllocate,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub struct ReviewProject {
        review_id: u32,
        project_id: u32,
    }
    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
        PackedLayout,
        SpreadAllocate,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub struct Review {
        id: u32,
        body: Vec<u8>,
        rating: u32,
        owner: AccountId,
    }
    #[derive(
        Debug,
        PartialEq,
        scale::Encode,
        scale::Decode,
        Clone,
        SpreadLayout,
        PackedLayout,
        SpreadAllocate,
        Default,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
    pub struct Project {
        review_count: u32,
        rating_sum: u32,
        owner: AccountId,
        meta: Vec<u8>,
        name: Vec<u8>,
    }

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the name already exists upon registration.
        ProjectAlreadyExists,
        /// Returned if caller is not owner while required to.
        ReviewAlreadyExists,
        /// Queried project does not exist
        ProjectDoesNotExist,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;
    impl Chocolate {
        /// Constructor that initializes the contract;
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> Result<()> {
            self.add_project("CHOC".bytes().collect(), Default::default())
        }

        /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get_projects(&self) -> Mapping<u32,Project> {
        //     // Not needed, iter. in ui https://substrate.stackexchange.com/questions/2562/how-to-iterate-over-mappingk-v?rq=1
        //     self.projects.into()
        // }
        /// Simply returns the current value of our `project`.
        #[ink(message)]
        pub fn get_project(&self, id: u32) -> Result<Project> {
            let maybe_project = self.projects.get(id);
            match maybe_project {
                None => Err(Error::ProjectDoesNotExist),
                Some(project) => Ok(project),
            }
        }
        #[ink(message)]
        pub fn add_project(&mut self, name: Vec<u8>, meta: Vec<u8>) -> Result<()> {
            let caller = self.env().caller();
            let index = self.project_index;
            let project = Project {
                owner: caller,
                name,
                meta,
                ..Default::default()
            };
            self.projects.insert(index, &project);
            self.project_index += 1;
            Ok(())
        }
        #[ink(message)]
        pub fn add_review(&mut self, body: Vec<u8>, rating: u32, project_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let review = Review {
                owner: caller,
                body,
                rating,
                ..Default::default()
            };
            let maybe_project = self.projects.get(project_id);
            match maybe_project {
                None => Err(Error::ProjectDoesNotExist),
                Some(_) => {
                    let key = (caller.clone(), project_id);
                    match self.reviews_projects_list.binary_search(&key) {
                        Err(index) => {
                            self.reviews_projects_list.insert(index, key);
                            let as_key_t: u32 =
                                index.try_into().expect("Vec should not exceed u32 size??"); // not sure.
                            self.reviews.insert(as_key_t, &review);
                            Ok(())
                        }
                        Ok(_) => Err(Error::ReviewAlreadyExists),
                    }
                }
            }
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn default_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink_env::test::set_caller::<Environment>(caller);
        }
        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Chocolate::default();
            assert_eq!(
                flipper.get_project(0),
                Ok(Project {
                    ..Default::default()
                })
            );
        }

        /// We test a simple use case of our contract. Adds a default project 0.
        #[ink::test]
        fn it_works() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut flipper = Chocolate::new();
            assert_eq!(flipper.get_project(0), Err(Error::ProjectDoesNotExist));
            flipper.flip().expect("Should add project 0");
            assert_eq!(
                flipper.get_project(0),
                Ok(Project {
                    owner: default_accounts.alice,
                    name: "CHOC".to_owned().into_bytes(),
                    meta: Default::default(),
                    ..Default::default()
                })
            );
            assert_eq!(flipper.project_index, 1);
        }
        
    }
}
