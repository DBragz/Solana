use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

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
    
    Ok(())
}
