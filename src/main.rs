use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
};
use std::time::Duration;
use tokio::time::sleep;

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
    
    // Get account balance (should be 0 for new account)
    match client.get_balance(&keypair.pubkey()) {
        Ok(balance) => {
            println!("💰 Account balance: {} lamports", balance);
        }
        Err(e) => {
            println!("⚠️  Could not fetch balance: {}", e);
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
