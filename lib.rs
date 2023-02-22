#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod chocolate {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::Storable;
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
        /// Stores the address of account that have initiated the verification flow.
        account_verification_flow_initiation: Mapping<AccountId, bool>, // TODO: Maybe use a bool?
        // Stores the count of verification attempts.
        verifications_count: u32,
        // Stores the addresses of accounts authorized to verify the identity.
        authorizers: Vec<AccountId>,
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

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
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
    #[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone)]
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
        /// Returned if the verification flow is already initiated.
        VerificationFlowAlreadyInitiated,
        /// Returned if the verification flow is not initiated.
        VerificationFlowNotInitiated,
        /// Return if the flow is not authorized.
        NotAuthorized,
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
                account_verification_flow_initiation: Default::default(),
                verifications_count: Default::default(),
                authorizers: Default::default(),
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
        #[ink(message)]
        pub fn add(&mut self) -> Result<()> {
            self.add_project("CHOC".bytes().collect(), Default::default(), Default::default(), Default::default())
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
        pub fn add_project(&mut self, name: Vec<u8>, meta: Vec<u8>, review_count: u32, rating_sum: u32) -> Result<()> {
            let caller = self.env().caller();
            let index = self.project_index;
            let project = Project {
                owner: caller,
                name,
                meta,
                review_count,
                rating_sum,
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

        #[ink(message)]
        pub fn initiate_verfication_flow(&mut self) -> Result<Vec<u8>> {
            // Cannot re-initiate flow if already begun
            if self
                .account_verification_flow_initiation
                .contains(&self.env().caller())
                {
                    return Err(Error::VerificationFlowAlreadyInitiated);
                }
            // Increment verification flow for uniqueness
            self.verifications_count = self.verifications_count.saturating_add(1);

            // Combine account and id for unique signable message
            let mut verification_message = self.env().caller().encode(&mut Vec::new());
            todo!("Add verifications_count to verification message");
            Ok(todo!("Sign verification message"))
        }

        #[ink(message)]
        pub fn verify_identity_response(&mut self, signature: [u8; 65]) -> Result<bool> {
            use sp_core::sr25519::Signature;

            // Ensure flow began
            if !self
                .account_verification_flow_initiation
                .contains(&self.env().caller())
                {
                    return Err(Error::VerificationFlowNotInitiated);
                }
            
            if !self.authorizers.contains(&self.env().caller()) {
                return Err(Error::NotAuthorized);
            }

            // Get ECDSA public key out of account - https://substrate.dev/rustdocs/v2.0.0/sp_core/crypto/trait.PublicKey.html

            // Verify using the caller's signature
            // let recovered_key ink_env::ecdsa_recover(signature, message_hash, output);
 
            // Check recovered key == candidate_pub_key

            // If true:
            Ok(true) 
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
            let mut chocolate = Chocolate::new(0);
            assert_eq!(chocolate.get_project(0), Err(Error::ProjectDoesNotExist));
            chocolate.add().expect("Should add project 0");
            assert_eq!(
                chocolate.get_project(0),
                Ok(Project {
                    owner: default_accounts.alice,
                    name: "CHOC".to_owned().into_bytes(),
                    meta: Default::default(),
                    review_count: 0,
                    rating_sum: 0,
                })
            );
            assert_eq!(chocolate.project_index, 1);
        }
        #[ink::test]
        fn it_works_review() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut chocolate = Chocolate::new(0);
            assert_eq!(
                chocolate.get_review(0, default_accounts.alice.clone()),
                Err(Error::ReviewDoesNotExist)
            );
            chocolate.add().expect("Should add test project 0");
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
