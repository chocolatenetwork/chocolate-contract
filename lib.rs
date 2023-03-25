#![cfg_attr(not(feature = "std"), no_std)]

// PackedLayout
#[ink::contract]
mod chocolate {
    use ink::env::hash::{Blake2x256, CryptoHash, HashOutput};
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use scale::Encode;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Chocolate {
        project_index: u32,
        /// Stores a project from id in storage
        projects: Mapping<u32, Project>,
        /// Index in reviews_projects_list array -> struct
        reviews: Mapping<u32, Review>,
        /// A list of review "keys", each review is identified by a unique Accountid + projectId pair.
        /// Where AccountId is the AccountId of the user who left the review, and the u32 is the projectedId of the project being reviewed.
        reviews_projects_list: Vec<(AccountId, u32)>,
        /// Stores the address of account that have initiated the verification flow.
        account_verification_flow_initiation: Mapping<AccountId, VerifyDetails>,
        /// Stores the count of verification attempts. Used to generate message for the next account_verification_flow_initiation entry.
        verifications_count: u32,
        /// Stores the addresses of accounts authorized to verify the identity.
        authorizers: Vec<AccountId>,
        /// Stores the addresses of accounts that have been verified.
        verified_accounts: Vec<AccountId>,
    }

    /// Information stored with a verification_flow_intiation.
    /// `index` the verifications_count when this verification_flow_initiation was created.
    /// `message` A unique combination of AccountId + index. set when the verification_flow_initiation entry is created.
    #[derive(Debug, PartialEq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
    )]
    pub struct VerifyDetails {
        index: u32,
        message: Vec<u8>,
    }

    /// A review left by a user.
    ///
    /// * `id`: Its key in `reviews`
    /// * `rating`: An integer from 1-5, with 5 being best and 1 being worst.
    /// * `owner`: The AccountId of the `User` who left the review.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
    )]
    pub struct Review {
        id: u32,
        rating: u32,
        owner: AccountId,
    }

    /// A project, associated with a single user
    ///
    /// * `review_count`: The number of reviews that have been left on this project.
    /// * `rating_sum`: The sum of the rating of the reviews on the project.
    /// * `meta`: Generic IPFS metadata associated with a project.
    /// Some Expected properties that this should have (in addition with what the `Project` struct already has) are shown [here](https://github.com/chocolatenetwork/choc-js/blob/main/packages/schema/schemas/project/project-schema.json)
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout,)
    )]

    pub struct Project {
        review_count: u32,
        rating_sum: u32,
        owner: AccountId,
        meta: Vec<u8>,
    }
    // Todo: Separate account types, into Account struct from spec. do create_user or create_project at verify
    /// Account Types. An enum used to identify a user as owning a project or a regular user.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum AccountType {
        /// User account
        User,
        /// Project Account
        Project,
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
        /// Cannot recover address using ecdsa_verify
        InvalidSignature,
        /// The account recovered does not match the address given
        VerificationFailed,
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;
    impl Chocolate {
        /// Constructor that initializes the contract;
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                project_index: 0,
                projects: Default::default(),
                reviews: Default::default(),
                reviews_projects_list: Default::default(),
                account_verification_flow_initiation: Default::default(),
                verifications_count: Default::default(),
                authorizers: Default::default(),
                verified_accounts: Default::default(),
            }
        }

        /// Return a project given it's id
        #[ink(message)]
        pub fn get_project(&self, id: u32) -> Result<Project> {
            let maybe_project = self.projects.get(id);
            match maybe_project {
                None => Err(Error::ProjectDoesNotExist),
                Some(project) => Ok(project),
            }
        }
        /// Return a review given the id of the project and the AccountId of the user who submitted it.
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

        /// Return list of projects (AccountId, ProjectId, project details(all fields))
        #[ink(message)]
        pub fn get_projects_list(&self) -> Vec<(AccountId, u32, Project)> {
            let mut projects_list: Vec<(AccountId, u32, Project)> = Vec::new();
            for id in 0..self.project_index {
                let project = self
                    .projects
                    .get(id)
                    .expect("Valid keys should have project entries");
                projects_list.push((project.owner, id, project));
            }
            projects_list
        }

        /// Return all reviews associated with a project given the project id.
        #[ink(message)]
        pub fn get_reviews_list(&self, project_id: u32) -> Vec<Review> {
            self.reviews_projects_list
                .iter()
                .enumerate()
                .filter(|(_, (_, id))| id.eq(&project_id))
                .map(|(index, _)| {
                    let as_key: u32 = index.try_into().expect("Expected valid index");
                    self.reviews.get(as_key).expect("Expected valid key")
                })
                .collect()
        }

        /// Return all users who have reviewed a project given the project id.
        #[ink(message)]
        pub fn get_reviewers_list(&self, project_id: u32) -> Vec<AccountId> {
            self.reviews_projects_list
                .iter()
                .filter(|s| s.1.eq(&project_id))
                .map(|s| s.0)
                .collect()
        }

        /// Return all projects reviewed by a user given the user's AccountId.
        #[ink(message)]
        pub fn get_projects_reviewed_by_user(&self, user: AccountId) -> Vec<(u32, Project)> {
            self.reviews_projects_list
                .iter()
                .filter(|s| s.0.eq(&user))
                .map(|s| {
                    let project = self
                        .projects
                        .get(s.1)
                        .expect("Valid keys should have project entries");
                    (s.1, project)
                })
                .collect()
        }

        /// Add a project to the contract's storage with some metadata (See (#Project)[`Project`])
        /// Initialise the project's fields to default values.
        #[ink(message)]
        pub fn add_project(&mut self, meta: Vec<u8>) -> Result<()> {
            let caller = self.env().caller();
            let index = self.project_index;
            let project = Project {
                owner: caller,
                meta,
                rating_sum: 0,
                review_count: 0,
            };
            self.projects.insert(index, &project);
            self.project_index += 1;
            Ok(())
        }

        /// Add a review to a project given it's rating and the projectId.
        #[ink(message)]
        pub fn add_review(&mut self, rating: u32, project_id: u32) -> Result<()> {
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
        /// Add someone to the list of authorizers
        #[ink(message)]
        pub fn add_authorizer(&mut self, authorizer: AccountId) -> Result<()> {
            match self.authorizers.binary_search(&authorizer) {
                Ok(_) => Ok(()),
                Err(index) => {
                    self.authorizers.insert(index, authorizer);
                    Ok(())
                }
            }
        }

        /// Generate a unique message composed of: `AccountId`(caller) + `index`, or return the existing message if it does not exist and create.
        #[ink(message)]
        pub fn initiate_verfication_flow(&mut self) -> Result<Vec<u8>> {
            // Cannot re-initiate flow if already begun
            let verify_details = self
                .account_verification_flow_initiation
                .get(&self.env().caller());

            match verify_details {
                Some(verify_detail) => Ok(verify_detail.message),
                None => {
                    // Increment verification flow for uniqueness

                    self.verifications_count = self.verifications_count.saturating_add(1);

                    // Combine account and id for unique signable message

                    let mut verification_message = self.env().caller().encode();
                    verification_message.extend_from_slice(&self.verifications_count.to_be_bytes());

                    let verify_detail: VerifyDetails = VerifyDetails {
                        index: self.verifications_count,
                        message: verification_message,
                    };
                    self.account_verification_flow_initiation
                        .insert(&self.env().caller(), &verify_detail);

                    Ok(verify_detail.message)
                }
            }
        }

        /// Takes the signature and the user who generated this signature and checks:
        ///
        /// * If the user exists
        /// * If the signature is valid (matches message and is a valid public key for the AccountId given.)
        #[ink(message)]
        pub fn verify_identity_response(
            &mut self,
            signature: [u8; 65],
            address_to_verify: AccountId,
        ) -> Result<()> {
            // Ensure is authorized
            if !self.authorizers.contains(&self.env().caller()) {
                return Err(Error::NotAuthorized);
            }
            // Ensure flow began

            let flow_init = self
                .account_verification_flow_initiation
                .get(&address_to_verify);

            if flow_init.is_none() {
                return Err(Error::VerificationFlowNotInitiated);
            }

            let details = flow_init.unwrap();

            // Verify using the caller's signature
            // https://substrate.dev/rustdocs/v2.0.0/sp_io/crypto/fn.secp256k1_ecdsa_recover.html

            // Hash the message to pass to ecdsa_recover
            // https://cryptobook.nakov.com/digital-signatures/ecdsa-sign-verify-messages

            let wrapped_message = Self::wrap_message(details.message);
            // The hash function that polkadotjs uses is Blake2x256, hence why Blake2x256 is used here.
            // Ref: https://github.com/polkadot-js/common/blob/4f5029118eb003041ff6c39c8336893a18590aee/packages/keyring/src/pair/index.ts#L38
            let message_hash: [u8; 32] = Self::hash_vec::<Blake2x256>(wrapped_message);

            let mut recovered: [u8; 33] = [0; 33];
            let recovered_result =
                ink::env::ecdsa_recover(&signature, &message_hash.into(), &mut recovered);

            // AccountId is recovered hashed with blake2x256: https://substrate.stackexchange.com/a/7448/88
            // Is encoding needed here?
            let output_as_account_id = Self::hash_vec::<Blake2x256>(recovered.into());
            // ink
            // Check recovered key == candidate_pub_key
            match recovered_result {
                Ok(_) => {
                    if address_to_verify == output_as_account_id.into() {
                        // Remove flow initiation
                        self.account_verification_flow_initiation
                            .remove(&self.env().caller());
                        // Add to verified accounts
                        self.verified_accounts.push(address_to_verify);
                        Ok(())
                    } else {
                        Err(Error::VerificationFailed)
                    }
                }
                Err(_) => Err(Error::InvalidSignature),
            }
        }

        /// Hash input which represents a message with ecdsa default hash.
        ///
        pub fn hash_vec<H>(input: Vec<u8>) -> <H as HashOutput>::Type
        where
            H: CryptoHash,
        {
            let mut output = <H as HashOutput>::Type::default(); // 256-bit buffer
            ink::env::hash_bytes::<H>(&input, &mut output);
            output
        }
        /// Wrap a message with <Bytes>..</Bytes> for hashing to verify a signature.
        pub fn wrap_message(input: Vec<u8>) -> Vec<u8> {
            let mut prefix: Vec<u8> = "<Bytes>".into();
            let suffix: Vec<u8> = "</Bytes>".into();
            // Check if we can avoid iterating over input here. And instead get <Bytes>..</Bytes> wrap by concatenation.
            prefix.extend(&input);
            prefix.extend(suffix);
            prefix
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
        /// We test if the new constructor does its job.
        #[ink::test]
        fn new_works() {
            let chocolate = Chocolate::new();
            assert_eq!(chocolate.get_project(0), Err(Error::ProjectDoesNotExist));
        }

        /// We test a simple use case of our contract. Adds a default project 0.
        #[ink::test]
        fn it_works() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut chocolate = Chocolate::new();
            assert_eq!(chocolate.get_project(0), Err(Error::ProjectDoesNotExist));
            chocolate
                .add_project(Default::default())
                .expect("Should add project 0");
            assert_eq!(
                chocolate.get_project(0),
                Ok(Project {
                    owner: default_accounts.alice,
                    meta: Default::default(),
                    rating_sum: Default::default(),
                    review_count: Default::default(),
                })
            );
            assert_eq!(chocolate.project_index, 1);
        }
        /// We test a simple use case of our contract. Adds a default project 0, and reviews it
        #[ink::test]
        fn it_works_review() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut chocolate = Chocolate::new();
            assert_eq!(
                chocolate.get_review(0, default_accounts.alice.clone()),
                Err(Error::ReviewDoesNotExist)
            );
            chocolate
                .add_project(Default::default())
                .expect("Should add test project 0");
            chocolate
                .add_review(10, 0)
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
                    rating: 10,
                    id: maybe_key.unwrap().try_into().expect("Should fit"),
                })
            )
        }

        /// Test starting the verification flow some account and verify that the message is as expected
        #[ink::test]
        fn initiate_verfication_flow_works() {
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut flipper = Chocolate::new();
            let message = flipper.initiate_verfication_flow();

            assert!(message.is_ok());

            assert_eq!(
                flipper
                    .account_verification_flow_initiation
                    .get(default_accounts.alice),
                Some(VerifyDetails {
                    index: 1,
                    message: message.unwrap(),
                })
            );
        }

        #[ink::test]
        fn verify_signature_works() {
            // Given
            let account: [u8; 32] = [
                143, 155, 234, 70, 218, 2, 174, 129, 205, 227, 158, 209, 53, 212, 224, 116, 8, 169,
                252, 94, 138, 7, 33, 174, 18, 183, 153, 90, 50, 61, 179, 31,
            ];
            let index = 1u32;
            let msg: [u8; 38] = [
                0, 144, 143, 155, 234, 70, 218, 2, 174, 129, 205, 227, 158, 209, 53, 212, 224, 116,
                8, 169, 252, 94, 138, 7, 33, 174, 18, 183, 153, 90, 50, 61, 179, 31, 0, 0, 0, 1,
            ];
            let sig: [u8; 65] = [
                4, 57, 253, 82, 44, 200, 18, 93, 199, 231, 124, 16, 136, 222, 120, 218, 156, 153,
                76, 94, 148, 29, 23, 233, 67, 170, 114, 59, 148, 152, 35, 246, 13, 169, 190, 60,
                107, 30, 59, 237, 106, 45, 29, 180, 180, 250, 139, 178, 251, 16, 216, 194, 69, 38,
                98, 154, 84, 12, 207, 70, 138, 248, 209, 241, 1,
            ];

            // And
            let default_accounts = default_accounts();
            let caller = default_accounts.alice;
            set_next_caller(caller.clone());
            let mut flipper = Chocolate::new();
            let details = VerifyDetails {
                index,
                message: msg.into(),
            };
            let account_as_account_id = AccountId::from(account);
            flipper
                .account_verification_flow_initiation
                .insert(account_as_account_id, &details);
            flipper.authorizers.push(caller.clone());
            // Then
            assert_eq!(
                flipper.verify_identity_response(sig, account_as_account_id.into()),
                Ok(())
            );
        }
    }
}
