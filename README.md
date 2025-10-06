# Smol Contracts

A Soroban smart contract for automated meme coin deployment and liquidity provision on the Stellar network.

## Overview

Smol Contracts is a Stellar Soroban contract that enables permissionless creation and trading of meme coins with automatic liquidity pool deployment. The contract integrates with Comet AMM pools to provide instant liquidity for newly created tokens, charging a small fee for minting and deploying.

### Key Features

- **Automated Token Deployment**: Creates Stellar Asset Contract (SAC) tokens from asset bytes
- **Instant Liquidity**: Deploys Comet AMM pools automatically for each new token
- **Batch Operations**: Support for creating multiple tokens in a single transaction
- **Batch Swaps**: Execute multiple swaps across different pools in one call
- **Admin Controls**: Upgradeable contract with configurable parameters
- **Fee Structure**: 100 base asset units (10 XLM) mint fee per token

## Architecture

### Contract Structure

```
.
├── contracts/
│   └── smol/
│       ├── src/
│       │   ├── lib.rs        # Main contract implementation
│       │   ├── comet.rs      # Comet AMM interface definitions
│       │   └── test.rs       # Test suite
│       └── Cargo.toml
├── ext/
│   └── comet.wasm           # Comet AMM contract binary
├── bindings/
│   └── smol-sdk/            # TypeScript bindings
├── Cargo.toml               # Workspace configuration
└── Makefile                 # Build automation
```

### Core Functions

#### `__constructor(admin, comet_wasm, base_asset)`
Initializes the contract with admin address, Comet WASM hash, and base asset address.

#### `coin_it(user, asset_bytes, salt, fee_rule) -> (Address, Address)`
Creates a new meme coin and deploys its liquidity pool:
- Deploys SAC token from `asset_bytes`
- Mints 10,000,000 tokens to the user
- Charges 100 base asset units (transferred to token issuer)
- Deploys Comet AMM pool with 99% token / 1% base asset ratio
- Returns tuple of (token_address, pool_address)

#### `coin_them(user, asset_bytes, salts, fee_rules) -> Vec<(Address, Address)>`
Batch version of `coin_it` for creating multiple tokens at once.

#### `swap_them_in(user, comet_addresses, tokens_out, token_amount_in, fee_recipients)`
Executes multiple swaps across different Comet pools in a single transaction.

#### `update(new_admin, new_comet_wasm, new_base_asset)`
Updates contract configuration (admin only).

#### `upgrade(wasm_hash)`
Upgrades contract to new WASM hash (admin only).

## Usage

### Prerequisites

- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli) installed
- Rust toolchain with `wasm32-unknown-unknown` target
- Make (optional, for convenience commands)

### Building

```bash
# Build and optimize the contract
make build

# Or manually
stellar contract build
stellar contract optimize \
  --wasm target/wasm32v1-none/release/smol.wasm \
  --wasm-out target/wasm32v1-none/optimized/smol.wasm
```

### Testing

```bash
make test
```

### Deployment

1. Upload the contract WASM:
```bash
stellar contract upload \
  --wasm target/wasm32v1-none/release/smol.wasm \
  --network testnet \
  --source default
```

2. Deploy the contract:
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/smol.wasm \
  --network testnet \
  --source default \
  -- \
  --admin <ADMIN_ADDRESS> \
  --comet_wasm <COMET_WASM_HASH> \
  --base_asset <BASE_ASSET_ADDRESS>
```

### Generating TypeScript Bindings

```bash
stellar contract bindings typescript \
  --wasm target/wasm32v1-none/release/smol.wasm \
  --output-dir bindings/smol-sdk \
  --overwrite
```

## Token Economics

Each token created through Smol has the following characteristics:

- **Initial Supply**: 10,000,000 tokens minted to creator
- **Pool Composition**:
  - 99% of supply (9,900,000 tokens) in pool
  - 100 base asset units (10 XLM) in pool
- **Fee Configuration**:
  - Dynamic fees between 5% and 95%
  - Based on tracked token utilization
  - Custom fee rules supported

## Dependencies

- [soroban-sdk](https://github.com/stellar/rs-soroban-sdk) (v23.0.2) - Core Soroban SDK
- [comet-factory](https://github.com/kalepail/comet-contracts-v1) - Comet AMM integration
- [soroban-fixed-point-math](https://github.com/kalepail/soroban-fixed-point-math) - Fixed point math utilities
- [itertools](https://github.com/rust-itertools/itertools) - Iterator helpers

## Development

### Formatting

```bash
make fmt
```

### Cleaning

```bash
make clean
```

### Creating Snapshots

The project includes ledger snapshot support for testing:

```bash
make snapshot
```

## Contract Addresses (Testnet)

See `CHEATSHEET` file for example deployment commands and addresses.

## Security Considerations

- Admin must be the asset issuer for minting through the contract
- Contract is upgradeable by admin
- All privileged operations require admin authorization
- Users must authorize token minting and fee payments

## License

See repository for license information.

## Resources

- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Comet AMM](https://github.com/kalepail/comet-contracts-v1)
