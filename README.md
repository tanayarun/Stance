<img src="https://github.com/tanayarun/Stance/blob/main/assets/PHOTO-2025-04-30-09-07-41.jpg?raw=true" height="200" width="200">

# Yield-Bearing Stablecoin — USDx

A **Soroban-based smart contract** implementing a **yield-bearing stablecoin** protocol inspired by [Ethena Labs' USDe](https://www.ethena.fi). USDx is a decentralized synthetic dollar backed by staked wXLM and designed to offer consistent, scalable yield without traditional bank intermediaries.

---

## Overview

This smart contract enables:

- **Staking of wXLM** to mint USDx (1:1 ratio)
- **Unstaking USDx** to get back your wXLM
- **Automatic Yield Distribution** to stakers
- **Claimable Rewards** in USDx
- **On-chain accounting of stake & rewards**
- **Trust-minimized, non-custodial staking**
- **Inspired by Ethena's internet-native dollar (USDe)**

---

## Core Concepts

### USDx
- A **synthetic stablecoin** minted by staking wXLM
- Soft-pegged to 1 USD
- Always fully backed by staked assets

### wXLM
- Wrapped XLM token used as the staking asset

### Yield Mechanism
- Yield is **simulated** and distributed proportionally to stakers
- Can be replaced with **real strategy returns** (e.g. LP profits, protocol fees)

---

## Features

### 1. `stake(user, amount)`
- Stakes `amount` of wXLM from `user`
- Mints `amount` of USDx to `user`
- Updates internal ledger and staker list

### 2. `unstake(user, amount)`
- Burns `amount` of USDx from `user`
- Transfers back `amount` of wXLM to `user`
- Updates stake records

### 3. `distribute_yield()`
- Distributes a simulated yield (e.g., 1000 USDx)
- Rewards are allocated proportionally based on user's staked amount

### 4. `claim_rewards(user)`
- Allows `user` to claim accumulated rewards in USDx
- Resets reward count to 0 after mint

### 5. `view_staked_balance(user) -> i128`
- Returns staked balance of `user`

### 6. `view_total_staked() -> i128`
- Returns total staked wXLM in the protocol

### 7. `view_accumulated_rewards() -> i128`
- Returns total unclaimed USDx rewards across all stakers

---

## Architecture

![architecture](https://raw.githubusercontent.com/tanayarun/Stance/refs/heads/main/assets/Untitled-2024-09-08-2251.png)

## Inspired By
- Ethena’s USDe: A synthetic dollar designed for the internet economy

- Real-world staking and yield systems such as:

- Lido (liquid staking)

- MakerDAO (collateral-backed stablecoins)

- Pendle/Element (yield tokenization)

## Disclaimer
This contract is for experimental use. Yield logic is currently simulated and not tied to real financial instruments. Always DYOR before deploying real funds.

## Future Improvements
- Integrate real LP yield via LiquidityPoolClient

- Add epoch-based yield streaming

- Implement APR-based minting rate modifiers

- Add frontend dashboard + analytics

## License
MIT License
