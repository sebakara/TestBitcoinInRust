
use serde_json::Value;
use std::error::Error;
use std::process::Command;

type AppResult<T> = Result<T, Box<dyn Error>>;

// Execute a Bitcoin Core RPC command.
fn rpc(wallet: Option<&str>, args: &[&str]) -> AppResult<String> {
    let mut command = Command::new("bitcoin-cli");
    command.arg("-regtest");

    if let Some(name) = wallet {
        command.arg(format!("-rpcwallet={name}"));
    }

    let output = command.args(args).output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            std::io::Error::new(
                error.kind(),
                "bitcoin-cli not found. Install Bitcoin Core and start `bitcoind -regtest` first.",
            )
        } else {
            error
        }
    })?;

    if !output.status.success() {
        return Err(
            String::from_utf8_lossy(&output.stderr)
                .to_string()
                .into()
        );
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

// Create a real Bitcoin Core wallet, or load it if it already exists.
fn create_wallet(name: &str) -> AppResult<()> {
    match rpc(None, &["createwallet", name]) {
        Ok(_) => println!("Created wallet: {name}"),
        Err(error) if error.to_string().contains("already exists") => {
            rpc(None, &["loadwallet", name]).ok();
            println!("Loaded wallet: {name}");
        }
        Err(error) => return Err(error),
    }
    Ok(())
}

// Generate a Bitcoin receiving address.
fn get_address(wallet: &str) -> AppResult<String> {
    rpc(Some(wallet), &["getnewaddress"])
}

// Mine test blocks to a specified address.
fn mine_blocks(count: u32, address: &str) -> AppResult<()> {
    let count_text = count.to_string();
    rpc(None, &["generatetoaddress", &count_text, address])?;
    Ok(())
}

// Send Bitcoin from one wallet to an address.
fn send_btc(
    from_wallet: &str,
    to_address: &str,
    amount: &str,
) -> AppResult<String> {
    rpc(
        Some(from_wallet),
        &["sendtoaddress", to_address, amount],
    )
}

// Retrieve the wallet's confirmed balance.
fn get_balance(wallet: &str) -> AppResult<String> {
    rpc(Some(wallet), &["getbalance"])
}

// Retrieve the actual transaction fee.
fn get_transaction_fee(
    wallet: &str,
    txid: &str,
) -> AppResult<f64> {
    let response = rpc(
        Some(wallet),
        &["gettransaction", txid],
    )?;

    let transaction: Value = serde_json::from_str(&response)?;

    let fee = transaction["fee"]
        .as_f64()
        .ok_or("Transaction fee not found")?;

    Ok(fee.abs())
}

fn main() -> AppResult<()> {
    // Step 1: Create wallets.
    create_wallet("class_miner")?;
    create_wallet("class_alice")?;
    create_wallet("class_bob")?;

    // Step 2: Generate test Bitcoin.
    let miner_address = get_address("class_miner")?;
    mine_blocks(101, &miner_address)?;

    // Step 3: Get receiving addresses.
    let alice_address = get_address("class_alice")?;
    let bob_address = get_address("class_bob")?;

    // Step 4: Deposit 5 BTC into Alice's wallet.
    send_btc("class_miner", &alice_address, "5.0")?;
    mine_blocks(1, &miner_address)?;

    println!("\n--- BEFORE TRANSFER ---");
    println!("Alice: {} BTC", get_balance("class_alice")?);
    println!("Bob:   {} BTC", get_balance("class_bob")?);

    // Step 5: Transfer 1.25 BTC from Alice to Bob.
    let txid = send_btc(
        "class_alice",
        &bob_address,
        "1.25",
    )?;

    // Step 6: Confirm the transaction.
    mine_blocks(1, &miner_address)?;

    // Step 7: Retrieve the transaction fee.
    let fee = get_transaction_fee("class_alice", &txid)?;

    println!("\n--- AFTER TRANSFER ---");
    println!("Transaction ID: {txid}");
    println!("Amount sent:    1.25 BTC");
    println!("Network fee:    {:.8} BTC", fee);
    println!("Fee in sats:    {:.0}", fee * 100_000_000.0);

    println!("\n--- FINAL BALANCES ---");
    println!("Alice: {} BTC", get_balance("class_alice")?);
    println!("Bob:   {} BTC", get_balance("class_bob")?);

    Ok(())
}
