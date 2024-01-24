#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod cgtoken {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct CgToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        staked_balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        AlreadyStaked,
        NotStaked,
        UnstakingPeriodNotElapsed,
    }

    impl CgToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self {
                total_supply,
                balances,
                staked_balances: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn staked_balance_of(&self, staker: AccountId) -> Balance {
            self.staked_balances.get(staker).map_or(0, |(balance, _)| balance)
        }

        #[ink(message)]
        pub fn stake(&mut self, amount: Balance) -> Result<(), Error> {
            let staker = self.env().caller();
            let balance = self.balance_of(staker);

            if amount > balance {
                return Err(Error::InsufficientBalance);
            }

            if self.staked_balance_of(staker) > 0 {
                return Err(Error::AlreadyStaked);
            }

            let timestamp = self.env().block_timestamp();

            self.balances.insert(staker, &(balance - amount));
            self.staked_balances.insert(staker, &(amount, timestamp));

            self.env().emit_event(Staked {
                staker,
                amount,
                timestamp,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn unstake(&mut self, amount: Balance) -> Result<(), Error> {
            let staker = self.env().caller();
            let (staked_balance, timestamp) = self.staked_balances.get(staker).ok_or(Error::NotStaked)?;

            if amount > *staked_balance {
                return Err(Error::InsufficientBalance);
            }

            let current_timestamp = self.env().block_timestamp();
            let unstaking_period = 14 * 24 * 60 * 60; // 14 days in seconds

            if current_timestamp < timestamp + unstaking_period {
                return Err(Error::UnstakingPeriodNotElapsed);
            }

            let balance = self.balance_of(staker);
            self.balances.insert(staker, &(balance + amount));
            self.staked_balances.insert(staker, &(staked_balance - amount, timestamp));

            self.env().emit_event(Unstaked { staker, amount });

            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn total_supply_works() {
            let cgtoken = CgToken::new(100);
            assert_eq!(cgtoken.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let cgtoken = CgToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(cgtoken.balance_of(accounts.alice), 100);
            assert_eq!(cgtoken.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn staking_works() {
            let mut cgtoken = CgToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 0);
            assert_eq!(cgtoken.stake(10), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 10);
        }

        #[ink::test]
        fn unstaking_works() {
            let mut cgtoken = CgToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 0);
            assert_eq!(cgtoken.stake(10), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 10);
            assert_eq!(cgtoken.unstake(5), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 5);
        }

        #[ink::test]
        fn unstaking_with_insufficient_balance_fails() {
            let mut cgtoken = CgToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 0);
            assert_eq!(cgtoken.stake(10), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 10);
            assert_eq!(cgtoken.unstake(15), Err(Error::InsufficientBalance));
        }

        #[ink::test]
        fn unstaking_before_period_fails() {
            let mut cgtoken = CgToken::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 0);
            assert_eq!(cgtoken.stake(10), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 10);
            assert_eq!(cgtoken.unstake(5), Ok(()));
            assert_eq!(cgtoken.staked_balance_of(accounts.alice), 5);
            assert_eq!(cgtoken.unstake(5), Err(Error::UnstakingPeriodNotElapsed));
        }
    }
}