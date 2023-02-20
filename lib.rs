#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod chocolate {
    use ink::storage::Mapping;

    use ink::prelude::vec::Vec;
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
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

    #[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
    )]
    pub struct ReviewProject {
        review_id: u32,
        project_id: u32,
    }

    #[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
    )]
    pub struct Review {
        id: u32,
        body: Vec<u8>,
        rating: u32,
        owner: AccountId,
    }
    #[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
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
        /// Queried Review does not exist
        ReviewDoesNotExist,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;
    impl Chocolate {
        // Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            Self {
                project_index: init_value,
                projects: Default::default(),
                reviews: Default::default(),
                reviews_projects_list: Default::default(),
            }
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
        pub fn flip(&mut self) -> Result<()> {
            self.add_project("CHOC".bytes().collect(), Default::default())
        }

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
        pub fn get_review(&self, project_id: u32, user: AccountId) -> Result<Review> {
            let maybe_index = self
                .reviews_projects_list
                .iter()
                .position(|s| s.0.eq(&user) && s.1.eq(&project_id));
            match maybe_index {
                None => Err(Error::ReviewDoesNotExist),
                Some(index) => {
                    let as_key: u32 = index.try_into().expect("Should fit");
                    let maybe_review = self.reviews.get(as_key);
                    let review = maybe_review.expect("Valid keys should have review entries");
                    Ok(review)
                }
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

            let maybe_project = self.projects.get(project_id);
            match maybe_project {
                None => Err(Error::ProjectDoesNotExist),
                Some(mut project) => {
                    let key = (caller.clone(), project_id);
                    match self.reviews_projects_list.binary_search(&key) {
                        Err(index) => {
                            let as_key_t: u32 =
                                index.try_into().expect("Vec should not exceed u32 size??"); // not sure.
                            project.rating_sum += rating;
                            project.review_count += 1;
                            let review = Review {
                                owner: caller,
                                body,
                                rating,
                                id: as_key_t,
                            };
                            self.reviews_projects_list.insert(index, key);
                            self.projects.insert(project_id, &project);
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
        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_next_caller(caller: AccountId) {
            ink::env::test::set_caller::<Environment>(caller);
        }
        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let chocolate = Chocolate::default();
            assert_eq!(chocolate.get_project(0), Err(Error::ProjectDoesNotExist));
        }

        /// We test a simple use case of our contract. Adds a default project 0.
        #[ink::test]
        fn it_works() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut chocolate = Chocolate::new();
            assert_eq!(chocolate.get_project(0), Err(Error::ProjectDoesNotExist));
            chocolate.flip().expect("Should add project 0");
            assert_eq!(
                chocolate.get_project(0),
                Ok(Project {
                    owner: default_accounts.alice,
                    name: "CHOC".to_owned().into_bytes(),
                    meta: Default::default(),
                    ..Default::default()
                })
            );
            assert_eq!(chocolate.project_index, 1);
        }
        #[ink::test]
        fn it_works_review() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut chocolate = Chocolate::new();
            assert_eq!(
                chocolate.get_review(0, default_accounts.alice.clone()),
                Err(Error::ReviewDoesNotExist)
            );
            chocolate.flip().expect("Should add test project 0");
            chocolate
                .add_review(Default::default(), 10, 0)
                .expect("Adding review should succeed");
            let maybe_key = chocolate
                .reviews_projects_list
                .iter()
                .position(|s| s.0.eq(&default_accounts.alice) && s.1.eq(&0));

            assert_eq!(maybe_key, Some(0)); // first key array.
            assert_eq!(
                chocolate.get_review(0, default_accounts.alice),
                Ok(Review {
                    owner: default_accounts.alice,
                    body: Default::default(),
                    rating: 10,
                    id: maybe_key.unwrap().try_into().expect("Should fit"),
                })
            )
        }
    }
}
