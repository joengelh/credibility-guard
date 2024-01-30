#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod platorm {

    use ink::{
        //codegen::EmitEvent,
        //reflect::ContractEventBase,
        prelude::vec::Vec,
        storage::Mapping,
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
        initial_pool: u128,
        claimed: bool,
        posted_at: Timestamp,
        betting_until: Timestamp,
        voting_until: Timestamp,
        bets_yes_promised: u128,
        bets_no_promised: u128,
        votes_yes: u128,
        votes_uncertain: u128,
        votes_no: u128,
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
        bettors: Mapping<(u128, AccountId), Bet>,
        voters: Mapping<(u128, AccountId), Vote>,
        counter: u128,
        fees_containing: u128,
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
            _cgtoken_code_hash: Hash,
        ) -> Self {
            let caller = Self::env().caller();
            let max_supply = 100000000;
            let cgtoken = CgTokenRef::new(max_supply)
                .code_hash(_cgtoken_code_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();
            Self {
                version: _version,
                owner: caller,
                post_fee: _post_fee,
                bet_fee: _bet_fee,
                betting_time: _betting_time,
                voting_time: _voting_time,
                counter: 0,
                bettors: Mapping::default(),
                voters: Mapping::default(),
                fees_containing: 0,
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
            let current_timestamp = Self::env().block_timestamp();
            let transferred_amount = self.env().transferred_value();
            assert_eq!(transferred_amount, self.post_fee + self.initial_pool);
            self.fees_containing += self.post_fee;
            self.counter += 1;
            let news = News {
                author: caller,
                pool: self.initial_pool,
                initial_pool: self.initial_pool,
                claimed: false,
                posted_at: current_timestamp,
                betting_until: current_timestamp + self.betting_time,
                voting_until: current_timestamp + self.betting_time + self.voting_time,
                bets_yes_promised: 0,
                bets_no_promised: 0,
                votes_yes: 0,
                votes_uncertain: 0,
                votes_no: 0,
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
            let caller = Self::env().caller();
            let current_timestamp = Self::env().block_timestamp();
            let mut news = self.news.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist"
                )
        });
        if let Some(_value) = self.bettors.get((id, caller)) {
            panic!("account already bet");
        }
        // check if voting is open
        assert!(news.betting_until < current_timestamp);
        let transferred_amount = self.env().transferred_value();
        assert!(transferred_amount > self.bet_fee);
        self.fees_containing += self.bet_fee;
        let amount = transferred_amount - self.bet_fee;
        let premium = calculate_premium(amount, direction, news.pool, news.bets_yes_promised, news.bets_no_promised);
        if direction {
            news.bets_yes_promised += premium;
        } else {
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
            if cast == 0 {
                news.votes_yes += 1;
            } else if cast == 1 {
                news.votes_no += 1; 
            } else if cast == 2 {
                news.votes_uncertain += 1;
            }
            else {
                panic!(
                    "illegal cast"
                )
            }
            // check if already voted
            if let Some(_value) = self.voters.get((id, caller)) {
                panic!("account already voted");
            }
            // check if voting is open
            assert!(news.voting_until > current_timestamp);
            assert!(news.betting_until < current_timestamp);
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
            // check if voting ended
            let caller = Self::env().caller();
            let current_timestamp = Self::env().block_timestamp();
            let news = self.news.get(id).unwrap_or_else(|| {
                // Contracts can also panic - this WILL fail and rollback the
                // transaction. Caller can still handle it and
                // recover but there will be no additional information about the error available. 
                // Use when you know something *unexpected* happened.
                panic!(
                    "broken invariant: expected entry to exist for the caller"
                )
            });
            let bettor = self.bettors.get((id, caller)).unwrap_or_else(|| {
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
                let _result = self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else if news.votes_yes > news.votes_no && bettor.direction == true {
                let _result = self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else if news.votes_yes < news.votes_no && bettor.direction == false {
                let _result = self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            } else if news.votes_yes > news.votes_no && bettor.direction == false {
                return 0;
            } else if news.votes_yes < news.votes_no && bettor.direction == true {
                return 0;
            } else {
                let _result = self.env().transfer(caller, bettor.amount_payed);
                return bettor.amount_promised;
            }
        }

        #[ink(message)]
        pub fn pool_claim(
            &mut self,
            id: u128,
        ) -> u128 {
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
            assert_eq!(news.author, caller);
            assert!(current_timestamp > news.voting_until);
            assert_eq!(news.claimed, false);
            let mut _payout = 0;
            if news.votes_uncertain > news.votes_yes && news.votes_uncertain > news.votes_no {
                _payout = news.initial_pool;
            } else if news.votes_yes > news.votes_no {
                _payout = news.pool - news.bets_yes_promised;
            } else if news.votes_yes < news.votes_no {
                _payout = news.pool - news.bets_no_promised;
            } else {
                _payout = news.initial_pool;
            }
            news.claimed = true;
            self.news.insert(id, &news);
            let _result = self.env().transfer(caller, _payout);
            return _payout;
        }

        #[ink(message)]
        pub fn fee_payout(
            &mut self,
        ) -> u128 {
            assert!(self.fees_containing > 0);
            let _fees_containing = self.fees_containing;
            let _result = self.env().transfer(self.owner, _fees_containing);
            self.fees_containing = 0;
            return _fees_containing;
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
        pub fn get_counter(&self) -> u128 {
            return self.counter;
        }

        #[ink(message)]
        pub fn get_fees_containing(&self) -> u128 {
            return self.fees_containing;
        }

        #[ink(message)]
        pub fn get_initial_pool(&self) -> u128 {
            return self.initial_pool;
        }

        #[ink(message)]
        pub fn get_all_news(&self) -> Vec<News> {
            let mut news_list = Vec::<News>::default();
            for n in 0..self.counter {
                let news: News = self.news.get(n).unwrap();
                news_list.push(news);
            }
            return news_list;
        }

        #[ink(message)]
        pub fn get_token(&self) -> CgTokenRef {
            return self.cgtoken.clone();
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
        let mut _offered_premium = 0;
        if choice {
            _offered_premium = ((pool - bets_yes_promised) * adjusted_weight) + amount;
        } else {
            _offered_premium = ((pool - bets_no_promised) * adjusted_weight) + amount;
        }
        return _offered_premium;
    }

    // This function calculates a percentage of a value
    fn percent_of_value(original_value: u128, reduction_percentage: u128) -> u128 {
        let reduced_value = original_value * reduction_percentage / 100 + 1;
        return reduced_value;
    }
}