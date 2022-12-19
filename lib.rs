#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink::storage::{traits::ManualKey, Mapping};
#[ink::contract]
mod flipper {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Chocolate {
        /// Stores a project from id in storage
        projects: Mapping<u32, Project>,
        // Multivalued keys would allow wider scope for reviews
        reviews: Mapping<u32, Review>,
        reviews_projects: Vec<Review_Project>
    }
    struct Review_Project{
        review_id: u32,
        project_id: u32,
    }
    struct Review {
        id: u32,
        body: Vec<u8>,
        rating: u32,
        owner: AccountId,
    }
    struct Project {
        review_count: u32,
        rating_sum: u32,
        owner: AccountId
    }
    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_projects: Mapping<u32,Project>, init_reviews: Mapping<u32,Review>, reviews_projects: Vec<Review_Project>) -> Self {
            Self { projects: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
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
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
