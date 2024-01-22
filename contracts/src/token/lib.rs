#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod mytoken {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Mytoken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl Mytoken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);
            Self {
                total_supply,
                balances,
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
            let mytoken = Mytoken::new(100);
            assert_eq!(mytoken.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let mytoken = Mytoken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(mytoken.balance_of(accounts.alice), 100);
            assert_eq!(mytoken.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut mytoken = Mytoken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(mytoken.balance_of(accounts.bob), 0);
            assert_eq!(mytoken.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(mytoken.balance_of(accounts.bob), 10);
        }
    }
}