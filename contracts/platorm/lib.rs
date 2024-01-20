#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod platorm {

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
        posted_at: BlockNumber,
        amount_pro: u32,
        amount_contra: u32,
        expires_at: BlockNumber,
        text: String,
        yes_token: AccountId,
        no_token: AccountId
    }

    #[ink(storage)]
    pub struct CredebilityGuard{
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
        pub fn new(
            _version: u8,
            _post_fee: Balance, 
            _vote_fee: Balance,
        ) -> Self {
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

        #[ink(message)]
        pub fn get_version(&self) -> u8 {
            return self.version;
        }

        /// Get owner of specific name.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            return self.owner;
        }
    }
}
