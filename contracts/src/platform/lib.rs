#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod platform {

    use ink::{
        codegen::EmitEvent,
        prelude::{format, string::String, vec::Vec},
        reflect::ContractEventBase,
        storage::Mapping,
        ToAccountId,
    };

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Proposal {
        author: AccountId,
        posted_at: u64,
        text: Hash,
        votes_approve: u64,
        votes_disprove: u64,
        votes_threshold: u64,
        approved: bool,
        metadata: String,
        end_timestamp: Timestamp,
    }

    #[ink(storage)]
    pub struct CredebilityGuard {
        version: u8,
        owner: AccountId,
        post_fee: Balance,
        vote_fee: Balance,
        counter: u32,
        proposal_map: Mapping<u32, Proposal>,
    }

    impl CredebilityGuard {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(_version: u8, _post_fee: Balance, _vote_fee: Balance) -> Self {
            let caller = Self::env().caller();
            let proposals = Mapping::default();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                vote_fee: _vote_fee,
                counter: 0u32,
                proposal_map: proposals,
            }
        }

        /// Get version
        #[ink(message)]
        pub fn get_version(&self) -> u8 {
            return self.version;
        }

        /// Get owner of specific name.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            return self.owner;
        }

        #[ink(message)]
        pub fn get_post_fee(&self) -> u128 {
            return self.post_fee;
        }

        #[ink(message)]
        pub fn get_vote_fee(&self) -> u128 {
            return self.vote_fee;
        }

        #[ink(message)]
        pub fn get_counter(&self) -> u32 {
            return self.counter;
        }

        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            _metadata: String,
            _text_hash: Hash,
            _end_timestamp: u64,
        ) -> u32 {
            self.counter = self.counter + 1;
            let caller = Self::env().caller();
            let proposal = Proposal {
                approved: false,
                author: caller,
                metadata: _metadata,
                posted_at: Self::env().block_timestamp(),
                votes_approve: 0,
                votes_disprove: 0,
                votes_threshold: 50,
                text: _text_hash,
                end_timestamp: _end_timestamp,
            };

            self.proposal_map.insert(self.counter, &proposal);

            return self.counter;
        }

        #[ink(message)]
        pub fn get_all_proposals(&self) -> Vec<Proposal> {
            let mut proposal_list = Vec::<Proposal>::default();
            for n in 0..self.counter {
                let proposal: Proposal = self.proposal_map.get(n).unwrap();
                proposal_list.push(proposal);
            }
            return proposal_list;
        }
    }
}
