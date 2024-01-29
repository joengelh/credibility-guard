# CredebilityGuard Smart Contract

![CredebilityGuard Logo](./credebility_guard_logo.png)

CredebilityGuard is a decentralized application (DApp) on the Aleo blockchain that allows users to create and participate in decentralized prediction markets. Users can post news articles, bet on the outcome of the news, and vote to determine the truth. The smart contract is implemented using the Ink! smart contract library for Rust.

## Features

- **Decentralized Prediction Markets:** CredebilityGuard allows users to create prediction markets for news articles. Users can post news, bet on the outcome, and vote to determine the truthfulness of the news.

- **Betting System:** Users can place bets on the outcome of news articles. The smart contract calculates premiums based on the amount of bets, providing an incentive for participants.

- **Voting Mechanism:** After the betting period, users can vote on the accuracy of the news. The voting threshold determines the percentage of agreement needed to determine the truth.

- **CgToken Integration:** CredebilityGuard integrates with CgToken, Aleo's native token, for staking and voting.

## Smart Contract Structure

The smart contract consists of three main structs:

1. **Bet:** Represents a user's bet on a news article, including the amount paid, promised premium, and direction (yes or no).

2. **Vote:** Represents a user's vote on the accuracy of a news article, including the amount staked and the cast (yes, no, or uncertain).

3. **News:** Represents a news article with details such as author, betting and voting periods, counters for yes and no bets, counters for yes and no promised premiums, vote counts, voting threshold, and metadata.

## Functions

- `post`: Post a news article and initialize the prediction market.
- `bet`: Place a bet on the outcome of a news article.
- `vote`: Vote on the accuracy of a news article.
- `claim`: Claim rewards after the voting period.

For a full list of functions, refer to the [smart contract code](./contracts/credebility_guard.rs).

## Configuration

- `post_fee`: Fee required to post a news article.
- `bet_fee`: Fee required to place a bet.
- `betting_time`: Duration of the betting period.
- `voting_time`: Duration of the voting period.
- `voting_threshold`: Percentage of agreement required for the news to be considered true.

## Owner Management

- `set_owner`: Change the owner of the smart contract.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

Happy predicting with CredebilityGuard! 🚀
