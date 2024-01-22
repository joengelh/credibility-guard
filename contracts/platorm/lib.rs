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
    pub struct Bet {
        amount: u128,
        // betters can bet yes or no
        direction: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Vote {
        amount: u128,
        // voters can vote yes, no or uncertain and can change their opinion
        direction: u8,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct News {
        author: AccountId,
        // State is an Enum that can be:
        // 0, meaning it is in the betting phase,
        // 1, meaning it is in the voting phase,
        // 2, meaning it was successfully voted on and approved to be true
        // 3, meaning it was successfully voted on and approved to be untrue
        // 4, meaning it was unsiccessfully voted on and the truth was not determined
        state: u8,
        posted_at: BlockNumber,
        betting_until: Timestamp,
        voting_until: Timestamp,
        betters: Mapping<AccountId, Bet>,
        bets_yes: u128,
        bets_no: u128,
        voters: Mapping<AccountId, Vote>,
        votes_yes: u128,
        votes_uncertain: u128,
        votes_no: u128,
        // voting threshold to determine the truth in decimal, so 0.5 means that 50% of voters have to agree
        voting_treshold: u8,
        metadata: String,
    }

    #[ink(storage)]
    pub struct CredebilityGuard {
        version: u8,
        owner: AccountId,
        post_fee: u128,
        vote_fee: u128,
        betting_time: u64,
        voting_time: u64,
        voting_treshold: u8,
        counter: u128,
        news_map: Mapping<u128, News>,
    }

    impl CredebilityGuard {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            _version: u8,
            _post_fee: u128, 
            _vote_fee: u128,
            _betting_time: u64,
            _voting_time: u64,
            _voting_treshold: u8,
        ) -> Self {
            let caller = Self::env().caller();
            let news = Mapping::default();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                vote_fee: _vote_fee,
                betting_time: _betting_time,
                voting_time: _voting_time,
                voting_treshold: _voting_treshold,
                counter: 0,
                news_map: news,
            }
        }

        #[ink(message, payable)]
        pub fn post(
            &mut self,
            _metadata: String,
        ) -> u128 {
            let caller = Self::env().caller();
            let current_block = Self::env().block_number();
            let current_timestamp = Self::env().block_timestamp();
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.post_fee);
            self.counter += 1;
            let news = News {
                author: caller,
                state: 0,
                posted_at: current_block,
                betting_until: current_timestamp + self.betting_time,
                voting_until: current_timestamp + self.betting_time + self.voting_time,
                betters: Mapping::default(),
                bets_yes: 0,
                bets_no: 0,
                voters: Mapping::default(),
                votes_yes: 0,
                votes_uncertain: 0,
                votes_no: 0,
                voting_treshold: self.voting_treshold,
                metadata: _metadata,
            };
            self.news_map.insert(self.counter, &news);
            return self.counter;
        }

        #[ink(message, payable)]
        pub fn bet(
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
            if direction {
                news.bets_yes += amount;
            } else {
                news.bets_no += amount; 
            }
            let caller = Self::env().caller();
            return (news.bets_yes, news.bets_no);
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
        pub fn get_betting_time(&self) -> u64 {
            return self.betting_time;
        }

        #[ink(message)]
        pub fn get_voting_time(&self) -> u64 {
            return self.voting_time;
        }

        #[ink(message)]
        pub fn get_voting_treshold(&self) -> u8 {
            return self.voting_treshold;
        }

        #[ink(message)]
        pub fn get_all_proposals(&self) -> Vec<News> {
            let mut news_list = Vec::<News>::default();
            for n in 0..self.counter {
                let news: News = self.news_map.get(n).unwrap();
                news_list.push(news);
            }
            return news_list;
        }

        #[ink(message)]
        pub fn set_owner(
            &self,
            address: AccountId
        ) -> AccountId {
            assert_eq!(self.owner, Self::env().caller());
            self.owner = address;
            return address;
        }

        #[ink(message)]
        pub fn set_post_fee(
            &self,
            post_fee: u128
        ) -> u128 {
            assert_eq!(self.owner, Self::env().caller());
            self.post_fee = post_fee;
            return post_fee;
        }

        #[ink(message)]
        pub fn set_vote_fee(
            &self,
            vote_fee: u128
        ) -> u128 {
            assert_eq!(self.owner, Self::env().caller());
            self.vote_fee = vote_fee;
            return vote_fee;
        }

        #[ink(message)]
        pub fn set_betting_time(
            &self,
            betting_time: u64,
        ) -> u64 {
            assert_eq!(self.owner, Self::env().caller());
            self.betting_time = betting_time;
            return betting_time;
        }

        #[ink(message)]
        pub fn set_voting_time(
            &self,
            voting_time: u64,
        ) -> u64 {
            assert_eq!(self.owner, Self::env().caller());
            self.voting_time = voting_time;
            return self.voting_time;
        }

        #[ink(message)]
        pub fn set_voting_treshold(
            &self,
            voting_treshold: u8,
        ) -> u8 {
            assert_eq!(self.owner, Self::env().caller());
            self.voting_treshold = voting_treshold;
            return self.voting_treshold;
        }
    }
}
