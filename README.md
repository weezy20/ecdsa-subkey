## Utility to see how AccountId20s are created from string seeds

When working with <a href="https://github.com/polkadot-evm/frontier.git">frontier</a> projects it's not clear how string seeds generate `AccountId20` values using ECDSA crypto. This is in contrast to working with `AccountId32` (sr25519 or ed25519) where you can simply verify the `AccountId32` of a given stringy seed using `subkey`.

This utility is a simple way to see how that happens for ECDSA keys.

### Usage
`cargo run -- <seed>`

This will print the `AccountId20` for the given seed, using the same `get_account_id_from_seed` used in substrate chainspec configs such as `local_testnet_config` or `developement_config`.