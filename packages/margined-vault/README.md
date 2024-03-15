# Vaultenator

<!-- dprint-ignore-start -->
[![GitHub Actions CI Workflow Status][1]][2]
[![Crates.io Version][3]][4]
<!-- dprint-ignore-end -->

An opinionated base vault implementing CosmWasm Vault Standard using
Tokenfactory.

## Features

- Administration - Handle opening and pausing contracts
- Configuration - Manage contract configuration
- Ownership - Manage contract ownership
- State - Manage contract state
- CW4626 - CosmWasm Vault Standard compliant interface
- Tokenfactory - Tokenfactory denoms as vault share tokens

## Usage

Add the crate to your CosmWasm project

```sh
cargo add vaultenator
```

To get started create a struct for your vault.

```rust
pub struct MyVault
```

Create Config and State structs.

```rust
#[cw_serde]
pub struct MyConfig {
    pub strategy_cap: Uint128,
    pub strategy_denom: Option<String>,
    pub base_denom: String,
}

#[cw_serde]
pub struct MyState {
    pub is_open: bool,
    pub is_paused: bool,
    pub last_pause: Timestamp,
}
```

Vaultenator provides traits with default implementations that can be overridden.

TODO FINISH THIS

[1]: https://img.shields.ifo/github/actions/workflow/status/margined-protocol/vaultenator/ci.yml?style=for-tTabel=ci
[2]: https://github.com/margined-protocol/vaultenator/actions/workflows/ci.yml
[3]: https://img.shields.io/crates/v/vaultenator?style=for-the-badge
[4]: https://crates.io/crates/vaultenator
