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

    use cgtoken::CgTokenRef;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Bet {
        amount_payed: u128,
        amount_promised: u128,
        claimed: bool,
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
        metadata: Hash,
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
        initial_pool: u128,
        news: Mapping<u128, News>,
        cgtoken: CgTokenRef,
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
            _inital_pool: u128,
            _voting_treshold: u8,
            _cgtoken_code_hash: Hash,
        ) -> Self {
            let cgtoken = CgTokenRef::new(true)
                .code_hash(_cgtoken_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();
            let caller = Self::env().caller();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                bet_fee: _bet_fee,
                betting_time: _betting_time,
                voting_time: _voting_time,
                voting_treshold: _voting_treshold,
                counter: 0,
                bettors: Mapping::default(),
                voters: Mapping::default(),
                fees_collected: 0,
                initial_pool: _inital_pool,
                news: Mapping::default(),
                cgtoken: cgtoken,
            }
        }

        #[ink(message, payable)]
        pub fn post(
            &mut self,
            _metadata: Hash,
        ) -> u128 {
            let caller = Self::env().caller();
            let current_block = Self::env().block_number();
            let current_timestamp = Self::env().block_timestamp();
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.post_fee + self.initial_pool);
            self.fees_collected += self.post_fee;
            self.counter += 1;
            let news = News {
                author: caller,
                pool: self.initial_pool,
                posted_at: current_block,
                betting_until: current_timestamp + self.betting_time,
                voting_until: current_timestamp + self.betting_time + self.voting_time,
                bets_yes_counter: 0,
                bets_no_counter: 0,
                bets_yes_promised: 0,
                bets_no_promised: 0,
                votes_yes: 0,
                votes_uncertain: 0,
                votes_no: 0,
                voting_treshold: self.voting_treshold,
                metadata: _metadata,
            };
            self.news.insert(self.counter, &news);
            let bettor = Bet {
                direction: true,
                amount_promised: 0,
                claimed: false,
                amount_payed: self.initial_pool,
            };
            self.bettors.insert((self.counter, caller), &bettor);
            return self.counter;
        }

        #[ink(message, payable)]
        pub fn bet(
            &mut self,
            direction: bool,
            id: u128,
        ) -> u128 {
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
        assert!(transferred_amount > self.bet_fee);
        self.fees_collected += self.bet_fee;
        let amount = transferred_amount - self.bet_fee;
        let premium = calculate_premium(amount, direction, news.pool, news.bets_yes_promised, news.bets_no_promised);
        if direction {
            news.bets_yes_counter += 1;
            news.bets_yes_promised += premium;
        } else {
            news.bets_no_counter += 1;
            news.bets_no_promised += premium;
        }
        let bet = Bet {
            amount_payed: amount,
            amount_promised: premium,
            claimed: false,
            direction: direction,
        };
        self.news.insert(id, &news);
        self.bettors.insert((id, caller), &bet);
        return amount;
        }

        #[ink(message)]
        pub fn vote(
            &mut self,
            cast: u8,
            id: u128,
        ) -> u128 {
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
            if cast == 0 {
                news.votes_yes += 1;
            } else if cast == 1 {
                news.votes_no += 1; 
            } else if cast == 2 {

            }
            else {
                panic!(
                    "illegal cast"
                )
            }
            let amount_staked = self.cgtoken.staked_balance_of(caller);
            let vote = Vote {
                amount_staked: amount_staked,
                cast: cast,
            };
            self.news.insert(id, &news);
            self.voters.insert((id, caller), &vote);
            return amount_staked;
        }


        #[ink(message)]
        pub fn claim(
            &mut self,
            id: u128,
        ) -> u128 {
            // check if the id exists
            assert!(self.counter >= id);
            // check if voting ended
            let caller = Self::env().caller();
            let current_timestamp = Self::env().block_timestamp();
            let mut news = self.news.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            let mut bettor = self.bettors.get((id, caller)).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            assert_eq!(bettor.claimed, false);
            assert!(news.voting_until < current_timestamp);
            if news.votes_uncertain > news.votes_yes && news.votes_uncertain > news.votes_no {
                self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else if news.votes_yes > news.votes_no && bettor.direction == true {
                self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else if news.votes_yes < news.votes_no && bettor.direction == false {
                self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else {
                self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            }
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
    fn calculate_premium(
        amount: u128,
        choice: bool,
        pool: u128,
        bets_yes_promised: u128,
        bets_no_promised: u128,
    ) -> u128 {
        let bet_weight = amount / pool;
        let adjusted_weight = percent_of_value(bet_weight, 95);
        let mut offered_premium = 0;
        if choice {
            offered_premium = ((pool - bets_yes_promised) * adjusted_weight) + amount;
        } else {
            offered_premium = ((pool - bets_no_promised) * adjusted_weight) + amount;
        }
        return offered_premium;
    }

    // This function calculates a percentage of a value
    fn percent_of_value(value: u128, percent: u128) -> u128 {
        // Convert u128 to f64 for the calculation
        let value_as_f64 = value as f64;

        // Calculate the reduced value
        let reduced_value = (value_as_f64 * percent as f64 / 100.0) as u128;

        // Return the reduced value
        reduced_value
    }
}