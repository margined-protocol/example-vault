# Example vault

<!-- dprint-ignore-start -->
[![GitHub Actions CI Workflow Status][1]][2]
[![GitHub Actions Build Workflow Status][3]][4]
<!-- dprint-ignore-end -->

An example vault to demonstrate [Vaultenator][1]

[1]: https://img.shields.io/github/actions/workflow/status/margined-protocol/example-vault/ci.yml?style=for-the-badge&label=ci
[2]: https://github.com/margined-protocol/example-vault/actions/workflows/ci.yml
[3]: https://img.shields.io/github/actions/workflow/status/margined-protocol/example-vault/build.yml?style=for-the-badge&label=build
[4]: https://github.com/margined-protocol/example-vault/actions/workflows/build.yml

## Installation

```sh
rustup show
cargo wasm # compile contract
cargo test # run tests
```

## Overview

This example vault showcases [Vaultenator][1] and offers a minimal
implementation of the [CosmWasm Vault Standard][2] with a TokenFactory used to
represent a share of the vault.

[Vaultenator][1] is based on traits and this minimal vault implements some of
these traits and uses the defaults for others. Implementers are free to add
their own implementations of any of these traits should functionality differ.

| Trait        | Default? | Implementation  |
| ------------ | -------- | --------------- |
| Administer   | Yes      |                 |
| Config       | No       | src/config.rs   |
| Describe     | No       | src/describe.rs |
| Handle       | No       | src/handle.rs   |
| ManageState  | No       | src/state.rs    |
| Own          | Yes      |                 |
| Query        | Yes      |                 |
| ReplyHandler | Yes      |                 |
| Vaultenator  | Yes      |                 |

[1]: https://github.com/margined-protocol/vaultenator
[2]: https://github.com/apollodao/cw-vault-standard
