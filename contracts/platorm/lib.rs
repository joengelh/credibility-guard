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
        amount_payed: u128,
        amount_promised: u128,
        // bettors can bet yes or no
        direction: bool,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Vote {
        amount_staked: u128,
        // voters can vote yes, no or uncertain and can change their opinion
        cast: u8,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct News {
        author: AccountId,
        pool: u128,
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
        bets_yes_counter: u128,
        bets_no_counter: u128,
        bets_yes_promised: u128,
        bets_no_promised: u128,
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
        bet_fee: u128,
        betting_time: u64,
        voting_time: u64,
        voting_treshold: u8,
        bettors: Mapping<(u128, AccountId), Bet>,
        voters: Mapping<(u128, AccountId), Vote>,
        counter: u128,
        fees_collected: u128,
        news: Mapping<u128, News>,
    }

    impl CredebilityGuard {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            _version: u8,
            _post_fee: u128, 
            _bet_fee: u128,
            _betting_time: u64,
            _voting_time: u64,
            _voting_treshold: u8,
        ) -> Self {
            let caller = Self::env().caller();
            let news = Mapping::default();
            let bettors = Mapping::default();
            let voters = Mapping::default();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                bet_fee: _bet_fee,
                betting_time: _betting_time,
                voting_time: _voting_time,
                voting_treshold: _voting_treshold,
                counter: 0,
                bettors: bettors,
                voters: voters,
                fees_collected: 0,
                news: news,
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
            self.fees_collected += self.post_fee;
            self.counter += 1;
            let news = News {
                author: caller,
                state: 0,
                posted_at: current_block,
                betting_until: current_timestamp + self.betting_time,
                voting_until: current_timestamp + self.betting_time + self.voting_time,
                bets_yes_counter: 0,
                bets_no_counter: 0,
                votes_yes: 0,
                votes_uncertain: 0,
                votes_no: 0,
                voting_treshold: self.voting_treshold,
                metadata: _metadata,
            };
            self.news.insert(self.counter, &news);
            return self.counter;
        }

        #[ink(message, payable)]
        pub fn bet(
            &mut self,
            direction: bool,
            amount: u128,
            id: u128,
        ) -> (u128, u128) {
            // check if the id exists
            assert!(self.counter >= id);
            let caller = Self::env().caller();
            let mut news = self.news.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.bet_fee + amount);
            self.fees_collected += self.bet_fee;
            if direction {
                news.bets_yes_counter += amount;
            } else {
                news.bets_no_counter += amount; 
            }
            let bet = Bet {
                amount_payed: amount,
                // bettors can bet yes or no
                direction: direction,
            };
            self.news.insert(id, &news);
            self.bettors.insert((id, caller), &bet);
            news.bets_no_promised += offered_premium;
        self.news.insert(news);
        self.bettors.insert(id, (amount, offered_premium, direction));
            return (news.bets_yes_counter, news.bets_no_counter);
        }

        
        #[ink(message)]
        pub fn vote(
            &mut self,
            cast: u8,
            id: u128,
        ) -> (u128, u128) {
            // check if the id exists
            assert!(self.counter >= id);
            let caller = Self::env().caller();
            let mut news = self.news.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            if (cast == 0) {
                news.votes_yes += 1;
            } else if (cast == 1) {
                news.bets_no_counter += amount; 
            } else if (cast == 2){

            }
            else {
                panic!(
                    "illegal cast"
                )
            }
            let vote = Vote {
                amount_staked: amount,
                cast: cast,
            };
            self.news.insert(id, &news);
            self.bettors.insert((id, caller), &bet);
            return (news.bets_yes_counter, news.bets_no_counter);
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
        pub fn get_bet_fee(&self) -> u128 {
            return self.bet_fee;
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
                let news: News = self.news.get(n).unwrap();
                news_list.push(news);
            }
            return news_list;
        }

        #[ink(message)]
        pub fn set_owner(
            & mut self,
            address: AccountId
        ) -> AccountId {
            assert_eq!(self.owner, Self::env().caller());
            self.owner = address;
            return address;
        }

        #[ink(message)]
        pub fn set_post_fee(
            &mut self,
            post_fee: u128
        ) -> u128 {
            assert_eq!(self.owner, Self::env().caller());
            self.post_fee = post_fee;
            return post_fee;
        }

        #[ink(message)]
        pub fn set_bet_fee(
            &mut self,
            bet_fee: u128
        ) -> u128 {
            assert_eq!(self.owner, Self::env().caller());
            self.bet_fee = bet_fee;
            return bet_fee;
        }

        #[ink(message)]
        pub fn set_betting_time(
            &mut self,
            betting_time: u64,
        ) -> u64 {
            assert_eq!(self.owner, Self::env().caller());
            self.betting_time = betting_time;
            return betting_time;
        }

        #[ink(message)]
        pub fn set_voting_time(
            &mut self,
            voting_time: u64,
        ) -> u64 {
            assert_eq!(self.owner, Self::env().caller());
            self.voting_time = voting_time;
            return self.voting_time;
        }

        #[ink(message)]
        pub fn set_voting_treshold(
            &mut self,
            voting_treshold: u8,
        ) -> u8 {
            assert_eq!(self.owner, Self::env().caller());
            self.voting_treshold = voting_treshold;
            return self.voting_treshold;
        }
    }

    // This function returns the money that will be won by the participant
    fn calculate_payout(
        &self,
        caller: AccountId,
        amount: u128,
        choice: bool,
        id: u128,
    ) -> u128 {
        let mut news = self.news.get(id).unwrap_or_else(|| {
            // Contracts can also panic - this WILL fail and rollback the
            // transaction. Caller can still handle it and
            // recover but there will be no additional information about the error available. 
            // Use when you know something *unexpected* happened.
            panic!(
                "broken invariant: expected entry to exist for the caller"
            )}
        );
        let bet_weight = amount / news.pool;
        news.pool += amount;
        if (choice) {
            let offered_premium = (((news.pool - news.bets_yes_promised) * bet_weight * 0.95) + bet_size);
        } else {
            let offered_premium = (((news.pool - news.bets_no_promised) * bet_weight * 0.95) + bet_size);
        }
        return offered_premium;
    }
        
}
