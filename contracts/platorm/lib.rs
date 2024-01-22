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
    pub struct News {
        author: AccountId,
        posted_at: BlockNumber,
        expires_at: BlockNumber,
        yes: Balance,
        no: Balance,
        text: String,
    }

    #[ink(storage)]
    pub struct CredebilityGuard{
        version: u8,
        owner: AccountId,
        post_fee: u128,
        vote_fee: u128,
        counter: u128,
        expires_after: u32,
        news_map: Mapping<u128, News>,
    }

    impl CredebilityGuard {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            _version: u8,
            _post_fee: u128, 
            _vote_fee: u128,
            expires_after: u32,
        ) -> Self {
            let caller = Self::env().caller();
            let news = Mapping::default();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                vote_fee: _vote_fee,
                counter: 0,
                expires_after: expires_after,
                news_map: news,
            }
        }
    
        #[ink(message, payable)]
        pub fn post(
            &mut self,
            text: String,
        ) -> u128 {
            let caller = Self::env().caller();
            let curr_block_number = Self::env().block_number();
            let expiry_block = curr_block_number + self.expires_after;
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.post_fee);
            self.counter += 1;
            let news = News {
                author: caller,
                posted_at: curr_block_number,
                expires_at: expiry_block,
                yes: 0,
                no: 0,
                text,
            };
            self.news_map.insert(self.counter, &news);
            return self.counter;
        }

        #[ink(message, payable)]
        pub fn vote(
            &self,
            direction: bool,
            amount: u128,
            id: u128,
        ) -> (u128, u128) {
            // check if the id exists
            assert!(self.counter >= id);
            let mut news = self.news_map.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.vote_fee + amount);
            if (direction) {
                news.yes += amount;
            } else {
                news.no += amount; 
            }
            let caller = Self::env().caller();
            return (news.yes, news.no);

        }

        #[ink(message)]
        pub fn get_version(&self) -> u8 {
            return self.version;
        }

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
        pub fn get_counter(&self) -> u128 {
            return self.counter;
        }

        #[ink(message)]
        pub fn get_expires_after(&self) -> u32 {
            return self.expires_after;
        }

    }
}
