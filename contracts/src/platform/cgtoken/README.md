# CgToken Smart Contract

![CgToken Logo](./cgtoken_logo.png)

CgToken is a simple and flexible ERC-20-like token smart contract implemented using the Ink! smart contract library for Rust. It provides basic token functionalities such as balance tracking, staking, and unstaking. The smart contract is designed to be a building block for more complex decentralized applications (DApps) on the Aleo blockchain.

## Features

- **Staking:** Users can stake their tokens to participate in various activities within the decentralized ecosystem.

- **Unstaking:** Staked tokens can be unstaked after a specific period, allowing users to regain liquidity.

- **Transfer:** Users can transfer tokens to other accounts, facilitating peer-to-peer transactions.

## Smart Contract Structure

The smart contract consists of the following key components:

1. **CgToken Struct:** Manages the total supply, user balances, staked balances, and staking timestamps.

2. **Error Enum:** Defines custom errors for various token-related operations, such as insufficient balance or attempting to unstake before the required period.

3. **Events (Staked, Unstaked):** Emit events to notify external systems about staking and unstaking activities.

## Functions

- `total_supply`: Get the total supply of CgTokens.

- `balance_of`: Get the balance of CgTokens for a specific account.

- `staked_balance_of`: Get the staked balance of CgTokens for a specific account.

- `staked_at`: Get the timestamp when a user staked their tokens.

- `stake`: Stake a specific amount of CgTokens.

- `unstake`: Unstake a specific amount of previously staked CgTokens.

- `transfer`: Transfer CgTokens to another account.

For a full list of functions, refer to the [smart contract code](./contracts/cgtoken.rs).

## Configuration

- The initial supply of CgTokens is set during contract deployment.

## Testing

The smart contract includes unit tests to ensure the correctness of its functionality. Run the tests using the following command:

```bash
cargo test --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

CgToken: Empowering decentralized ecosystems with staking flexibility! 🚀
