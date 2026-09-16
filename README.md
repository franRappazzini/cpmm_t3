# CPMM (Constant Product Market Maker)

An CPMM program implemented on Solana using the Anchor framework. This project implements a liquidity pool system with token swapping based on the constant product formula (x \* y = k).

## 🚀 Features

- **Protocol Initialization**: Configuration of protocol and swap fees
- **Liquidity Pool Creation**: Create pools for token pairs
- **Liquidity Deposits**: Users can deposit tokens and receive LP tokens
- **Liquidity Withdrawals**: Redeem LP tokens for underlying tokens
- **Token Swaps**: Exchange tokens with slippage protection
- **Protocol Fees**: Fee system with withdrawal capability

## 📋 Prerequisites

Before starting, make sure you have installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/)
- [Yarn](https://yarnpkg.com/getting-started/install)

### Verify installations

```bash
rustc --version
solana --version
anchor --version
node --version
yarn --version
```

## 🔧 Installation

### 1. Clone the repository

```bash
git clone git@github.com:franRappazzini/cpmm-t3.git
cd cpmm-t3
```

### 2. Install dependencies

```bash
yarn install
```

### 3. Configure Solana CLI

Set up your Solana wallet and make sure you're on the localnet network:

```bash
# Create a new wallet if you don't have one
solana-keygen new

# Set cluster to localnet
solana config set --url localhost
```

## 🧪 Local Testing

### 1. Build the program

```bash
anchor build
```

### 2. Run the tests

Run all tests:

```bash
anchor test
```

## 🔑 Program Instructions

### Initialize

Initializes the protocol's global configuration with fees.

### Create Pool

Creates a new liquidity pool for a token pair.

### Add Liquidity

Allows users to deposit tokens into a pool and receive LP tokens.

### Redeem LP

Allows users to burn LP tokens and receive the underlying tokens.

### Swap

Allows swapping one token for another with slippage protection.

### Withdraw Treasury

Allows withdrawing accumulated protocol fees.

## 📐 Mathematical Model

The CPMM uses the constant product formula:

```
x * y = L
```

Where:

- `x` = amount of token A in the pool
- `y` = amount of token B in the pool
- `L` = liquidity constant

**Note**: This project is configured to work on `localnet` by default. To deploy on devnet or mainnet, modify the configuration in `Anchor.toml`.
