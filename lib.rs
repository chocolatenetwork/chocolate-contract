#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
// PackedLayout
#[ink::contract]
mod flipper {
    use ink_storage::{ Mapping};
    use ink_storage::{traits::{SpreadLayout,PackedLayout}};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(Default)]
    #[ink(storage)]
    pub struct Chocolate {
        /// Stores a project from id in storage
        projects: Mapping<u32, Project>,
        // Multivalued keys would allow wider scope for reviews
        reviews: Mapping<u32, Review>,
        reviews_projects: Vec<ReviewProject>
    }
     #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode, Debug,Default)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    struct ReviewProject{
        review_id: u32,
        project_id: u32,
    }
     #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode,Default)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    struct Review {
        id: u32,
        body: Vec<u8>,
        rating: u32,
        owner: AccountId,
    }
    #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode,Default, Debug,PartialEq)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct Project {
        review_count: u32,
        rating_sum: u32,
        owner: AccountId
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
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Default::default()
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self)-> Result<()> {
            let s = Project {
                review_count: Default::default(),
                rating_sum: Default::default(),
                owner: Default::default(),
            };
            match self.projects.get(&0) {
                Some(_) => Err(Error::ProjectAlreadyExists),
                _ => {
                    self.projects.insert(&0, &s);
                    Ok(())
                }
            }
        }

        /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get_projects(&self) -> Mapping<u32,Project> {
        //     // Not needed, iter. in ui https://substrate.stackexchange.com/questions/2562/how-to-iterate-over-mappingk-v?rq=1
        //     self.projects.into()
        // }
        /// Simply returns the current value of our `project`.
        #[ink(message)]
        pub fn get_project(&self,id: u32) -> Result<Project> {
            let maybe_project = self.projects.get(id);
            match maybe_project {
                None => Err(Error::ProjectDoesNotExist),
                Some(project)=> {
                    Ok(project)
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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Chocolate::default();
            assert_eq!(flipper.get_project(0), Ok(Project {
                ..Default::default()
            }));
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Chocolate::new();
            assert_eq!(flipper.get_project(0), Err(Error::ProjectDoesNotExist));
            flipper.flip().expect("Should add project 0");
            assert_eq!(flipper.get_project(0), Ok(Project {
                review_count: Default::default(),
                rating_sum: Default::default(),
                owner: Default::default(),
            }));
        }
    }
}
