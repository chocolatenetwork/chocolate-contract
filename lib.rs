#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
// PackedLayout
#[ink::contract]
mod chocolate {
    use ink_eth_compatibility::ECDSAPublicKey;
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout};
    use ink_storage::Mapping;
    use scale::Encode;
    // scale::Encode::encode(&self.env().caller(), &mutverification_message)
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
    pub struct VerifyDetails {
        index: u32,
        message: Vec<u8>,
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
    // Todo: Separate account types, into Account struct from spec. Add create_user and create_project at verify
    /// Account Types
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
        /// Invalid signature
        InvalidSignature,
        /// Invalid message
        VerificationFailed,
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
        // Add someone to the list of authorizers
        #[ink(message)]
        pub fn add_authorizer(&mut self, authorizer: AccountId) -> Result<()> {

            match self.authorizers.binary_search(&authorizer){
                Ok(_)=>{
                    Ok(())
                }
                Err(index) =>{
                    self.authorizers.insert(index, authorizer);
                    Ok(())
                }
            }
        }

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

        #[ink(message)]
        pub fn verify_identity_response(
            &mut self,
            signature: [u8; 65],
            address_to_verify: AccountId,
        ) -> Result<bool> {
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
            let message_hash: [u8; 32] = Self::hash_vec(details.message);
            
            let mut recovered: [u8; 33] = [0; 33]; 
            let recovered_result = ink_env::ecdsa_recover(&signature, &message_hash.into(), &mut recovered);


            let ecdsa_output: ECDSAPublicKey = recovered.into();
            let output_as_account_id = ecdsa_output.to_default_account_id();

            // ink
            // Check recovered key == candidate_pub_key
            match recovered_result {
                Ok(_) => {
                    if address_to_verify == output_as_account_id {
                        // Remove flow initiation
                        self.account_verification_flow_initiation
                            .remove(&self.env().caller());
                        // Add to verified accounts
                        self.verified_accounts
                            .push(address_to_verify);
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                Err(_) => Err(Error::VerificationFailed),
            }
        }

        pub fn hash_vec(input: Vec<u8>) -> [u8; 32] {
            use ink_env::hash::{HashOutput, Sha2x256};
            let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink_env::hash_bytes::<Sha2x256>(&input, &mut output);
            output
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
        #[ink::test]
        fn initiate_verfication_flow_works(){
             
            let default_accounts = default_accounts();
            set_next_caller(default_accounts.alice);
            let mut flipper = Chocolate::new();
            let message =  flipper.initiate_verfication_flow();

            assert!(message.is_ok());

            assert_eq!(flipper.account_verification_flow_initiation.get(default_accounts.alice), Some(VerifyDetails {
                index: 1,
                message: message.unwrap(),
            }));
        }
    }
}
