# Bitcoin Wallet Demo

A small Rust program that talks to a local **Bitcoin Core regtest** node through `bitcoin-cli`. It creates three wallets, mines test coins, sends BTC from Alice to Bob, then prints balances and the network fee.

This uses **regtest only**. It does not touch mainnet or spend real bitcoin.

## What it does

1. Creates (or loads) wallets: `class_miner`, `class_alice`, `class_bob`
2. Mines 101 blocks to the miner so coinbase rewards become spendable
3. Sends **5 BTC** from the miner to Alice
4. Sends **1.25 BTC** from Alice to Bob
5. Mines confirmation blocks and prints the transaction fee and final balances

Expected shape of the output:

```
--- BEFORE TRANSFER ---
Alice: 5.00000000 BTC
Bob:   0.00000000 BTC

--- AFTER TRANSFER ---
Transaction ID: ...
Amount sent:    1.25 BTC
Network fee:    0.00002820 BTC
Fee in sats:    2820

--- FINAL BALANCES ---
Alice: 3.74997180 BTC
Bob:   1.25000000 BTC
```

Exact fee and remaining Alice balance can vary slightly.

## Requirements

- [Rust](https://rustup.rs/) (this crate uses edition 2024)
- [Bitcoin Core](https://bitcoincore.org/) (`bitcoind` and `bitcoin-cli` on your `PATH`)

On macOS with Homebrew:

```bash
brew install bitcoin
```

Confirm the binaries exist:

```bash
which bitcoind bitcoin-cli
```

## Run

Start a **regtest** node with a fallback fee. A brand-new chain has no fee-estimation history, so `sendtoaddress` fails unless `-fallbackfee` is set:

```bash
bitcoind -regtest -daemon -fallbackfee=0.0002
```

Then:

```bash
cargo run
```

Stop the node when you are done:

```bash
bitcoin-cli -regtest stop
```

Do **not** use `brew services start bitcoin` for this demo. That starts a mainnet node, not regtest.

## Troubleshooting

**`bitcoin-cli not found`**  
Bitcoin Core is not installed, or `bitcoin-cli` is not on `PATH`. Install it and check `which bitcoin-cli`.

**`Fee estimation failed. Fallbackfee is disabled.`**  
Restart the node with `-fallbackfee=0.0002` as shown above.

**`Could not connect to the server`**  
`bitcoind` is not running. Start it in regtest mode first.

Wallets persist under Bitcoin Core’s regtest data directory. A second `cargo run` loads the existing wallets instead of creating them again.
