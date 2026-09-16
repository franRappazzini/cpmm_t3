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

Here is a structured, bulleted response in English that you can use or adapt directly for your assignment submission:

---

## How to Mitigate Downtime

To ensure high availability and prevent downtime for DeFi protocols, mitigation strategies must be applied across all key layers of the stack:

#### 1. Infrastructure & RPC Node Resilience

- **Multi-RPC Failover Strategy:** Implement automated fallback logic in the client/frontend. If the primary RPC provider (e.g., Triton, Helius, QuickNode, Alchemy) experiences an outage or rate-limiting, the application automatically switches to secondary and tertiary providers.
- **Load Balancing & Caching:** Distribute read requests across multiple RPC nodes and cache non-critical state data locally to reduce direct RPC load.

---

#### 2. Frontend Availability & Decentralization

- **Decentralized Hosting:** Deploy the frontend to decentralized storage networks such as **IPFS** or **Arweave** linked to decentralized domain systems (ENS / SNS). This prevents single points of failure from traditional web hosts (AWS, Vercel, Cloudflare).
- **Open-Source & Self-Hosting Support:** Maintain an open-source frontend repository allowing users and third parties to build and run the interface locally or host backup mirrors in the event of an official site outage.

---

#### 3. Program & Oracle Reliability

- **Oracle Redundancy & TWAP Fallbacks:** If the CPMM or DeFi protocol relies on external price feeds, integrate redundant oracle providers (e.g., Pyth + Chainlink) and implement on-chain Time-Weighted Average Price (TWAP) fallbacks if external oracle feeds freeze or fail.
- **Non-Blocking Architecture:** Avoid single-point emergency pause functions (`pause()` or single admin keys) that could permanently lock protocol funds if admin keys are compromised or inaccessible during network emergencies.
- **Graceful Degradation:** Design instructions so that if a non-critical Cross-Program Invocation (CPI) or external module fails, core protocol functions (e.g., basic swaps or liquidity removals) remain functional.

---

#### 4. Network Congestion & Execution Reliability

- **Dynamic Priority Fees & Compute Budgeting:** During periods of extreme market volatility, network blocks become congested. Implement dynamic priority fee algorithms within the client to automatically adjust transaction fees and compute unit budgets, ensuring user transactions are accepted into blocks without timing out.
- **Transaction Retry & Status Polling Logic:** Build robust client-side retry mechanisms with exponential backoff to handle dropped or unconfirmed transactions seamlessly during periods of high network load.
