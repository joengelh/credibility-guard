# credibility-guard

## Why credibility guard?

Credibility Guard, our decentralized app on Aleph Zero, tackles the fake news dilemma by engaging users in a dynamic truth-seeking process. Users upload suspicious news, initiating a prediction market where bettors stake on its authenticity. Voters, economically incentivized to uphold truth, stake platform tokens. The news uploader pays fees, profiting as more users engage.

Bettors enjoy market-adjusted rates and payouts based on collective predictions, creating a system where accuracy is financially rewarded. Credibility Guard transforms users into guardians of credibility, fostering a transparent information landscape and incentivizing active participation in the fight against misinformation.

## Game Theory

Credibility Guard leverages game theory to create a powerful weapon against fake news. In this decentralized app on Aleph Zero, users engage in a strategic dance of prediction and validation.

The game begins with individuals submitting news they suspect to be fake, triggering a prediction market. Bettors, armed with market-adjusted rates, strategically place their stakes on the news' authenticity. Simultaneously, voters, economically incentivized by staking platform tokens, cast their ballots, each move impacting the token's value.

The uploader, entering the game by paying initial liquidity and a small fee, stands to profit as more users engage. This dynamic creates a compelling interplay of incentives, where honesty is not just encouraged but economically enforced.

Bettors, skilled in forecasting the future, reap rewards based on collective predictions. The game unfolds as participants actively contribute to a more truthful digital landscape, earning profits by aligning with reality.

Credibility Guard, driven by game theory, transforms the fight against fake news into a strategic and rewarding endeavor, where every move contributes to a more trustworthy information ecosystem.

## Deployments on Testnet

- Token
  {
  "address": "",
  "contractHash": "0x4e85c1063e00a5678c87b353c56944b205e6a1084d4b92d54cd40b1a793cb9ef",
  "codeHash": "0xe3f612e6b055f851100edc2a2426fe5b36ee8600d5a8e89c53848657e4fa847a",
  }
- Platform
  {
  "address": "",
  "contractHash": "0x2048b56deb1baa7b74db7170c8d97e239963df3f993e95613053a5d9f1ea325b",
  "codeHash": "0xadbdfa686aefc020d6816fe7d08be55ed89c9b6689f59eb3aee0d8471e551ad4",
  "deploymentSalt": "0x5f80a658f3b7568e60ca5d3a96463f1ace623cacebe62396004f55d960a7a0ae",
  }

## Install

cargo install ink-wrapper --locked --force --version 0.4.1

## Build

smart contracts from their respective repos:

```bash
cargo +nightly contract build --release
```
