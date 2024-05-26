# Token Contract with Freeze Functionality

## Overview

This project is a modified version of a token contract with added functionalities to freeze and unfreeze accounts. When an account is frozen, it cannot transfer tokens.

## Features

- **Freeze Account**: Allows freezing a specified account to prevent token transfers.
- **Unfreeze Account**: Allows unfreezing a specified account to re-enable token transfers.
- **Transfer**: Checks if the sender's account is frozen before transferring tokens.

## Contract Address

Deployed on Stellar testnet: `[Your Contract Address Here]`

## Instructions

### 1. Compile the Contract

To compile the contract, use the following command:

```sh
cargo build --target wasm32-unknown-unknown --release
