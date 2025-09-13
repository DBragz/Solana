use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::time::Duration;
use tokio::time::sleep;
use std::str::FromStr;

// Function to write counter to blockchain using memo program
async fn write_counter_to_blockchain(
    client: &RpcClient,
    payer: &Keypair,
    counter: u32,
) -> Result<String, Box<dyn std::error::Error>> {
    // Solana Memo Program ID
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    
    // Create memo data with counter
    let memo_data = format!("Counter: {} - Written to blockchain at {}", counter, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    
    // Create memo instruction
    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![
            AccountMeta::new_readonly(payer.pubkey(), true),
        ],
        data: memo_data.as_bytes().to_vec(),
    };
    
    // Get recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;
    
    // Create transaction
    let transaction = Transaction::new_signed_with_payer(
        &[memo_instruction],
        Some(&payer.pubkey()),
        &[payer],
        recent_blockhash,
    );
    
    // Send transaction
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("📝 Blockchain Transaction: {} | Counter: {} | Signature: {}", memo_data, counter, signature);
            Ok(signature.to_string())
        }
        Err(e) => {
            println!("❌ Transaction failed: {}", e);
            Err(e.into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Solana Application Starting...");
    
    // Connect to Solana devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    
    // Check cluster version
    match client.get_version() {
        Ok(version) => {
            println!("✅ Connected to Solana cluster!");
            println!("📋 Cluster version: {}", version.solana_core);
        }
        Err(e) => {
            println!("❌ Failed to connect to Solana cluster: {}", e);
            return Err(e.into());
        }
    }
    
    // Generate a new keypair
    let keypair = Keypair::new();
    println!("🔑 Generated new keypair with public key: {}", keypair.pubkey());
    
    // Get initial account balance
    let initial_balance = client.get_balance(&keypair.pubkey()).unwrap_or(0);
    println!("💰 Initial account balance: {} lamports", initial_balance);
    
    // Request airdrop if account is empty (needed for transaction fees)
    if initial_balance == 0 {
        println!("🪂 Requesting airdrop of 1 SOL for transaction fees...");
        match client.request_airdrop(&keypair.pubkey(), 1_000_000_000) {
            Ok(signature) => {
                println!("✅ Airdrop requested! Signature: {}", signature);
                // Wait a bit for the airdrop to process
                println!("⏳ Waiting for airdrop to complete...");
                sleep(Duration::from_secs(5)).await;
                
                match client.get_balance(&keypair.pubkey()) {
                    Ok(new_balance) => {
                        println!("💰 Updated account balance: {} lamports ({:.9} SOL)", new_balance, new_balance as f64 / 1_000_000_000.0);
                    }
                    Err(e) => println!("⚠️  Could not fetch updated balance: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Airdrop failed: {}", e);
                println!("⚠️  Continuing without funds - transactions may fail");
            }
        }
    }
    
    println!("🎉 Solana application running successfully!");
    println!("🌐 Visit https://explorer.solana.com/?cluster=devnet to explore the Solana devnet");
    println!("\n⏰ Starting continuous monitoring... (Press Ctrl+C to stop)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let mut update_counter = 1;
    let mut account_counter = 0;  // Account counter that will stop at 10
    
    loop {
        // Increment account counter before refresh, but stop at 10
        if account_counter < 10 {
            account_counter += 1;
            println!("\n🔢 Account Counter: {} (incremented before refresh)", account_counter);
            
            // Write counter to blockchain using memo program
            if let Err(e) = write_counter_to_blockchain(&client, &keypair, account_counter).await {
                println!("❌ Failed to write counter to blockchain: {}", e);
            } else {
                println!("✅ Successfully wrote counter {} to blockchain!", account_counter);
            }
        } else {
            println!("\n🔢 Account Counter: {} (stopped at 10, no longer incrementing)", account_counter);
        }
        
        println!("📊 Update #{} - {}", update_counter, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
        
        // Check cluster health
        match client.get_health() {
            Ok(_) => println!("💚 Cluster Status: Healthy"),
            Err(e) => println!("💔 Cluster Status: Issues detected - {}", e),
        }
        
        // Get latest blockhash
        match client.get_latest_blockhash() {
            Ok(blockhash) => println!("🧱 Latest Blockhash: {}", blockhash),
            Err(e) => println!("⚠️  Could not fetch latest blockhash: {}", e),
        }
        
        // Show account balance again
        match client.get_balance(&keypair.pubkey()) {
            Ok(balance) => {
                if balance > 0 {
                    println!("💰 Account Balance: {} lamports ({:.9} SOL)", balance, balance as f64 / 1_000_000_000.0);
                } else {
                    println!("💰 Account Balance: 0 lamports (empty account)");
                }
            }
            Err(e) => println!("⚠️  Could not fetch balance: {}", e),
        }
        
        // Get slot information
        match client.get_slot() {
            Ok(slot) => println!("🎰 Current Slot: {}", slot),
            Err(e) => println!("⚠️  Could not fetch slot: {}", e),
        }
        
        println!("\n⏳ Waiting 10 seconds before next update...");
        sleep(Duration::from_secs(10)).await;
        
        update_counter += 1;
        
        // Optional: limit to prevent infinite running for demo purposes
        if update_counter > 100 {
            println!("\n🔄 Reached 100 updates. Restarting update counter...");
            update_counter = 1;
        }
    }
}
