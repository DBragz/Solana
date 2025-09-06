# Solana Blockchain Application

## Overview
A Rust-based Solana blockchain application that connects to the Solana devnet and demonstrates basic blockchain interactions.

## Project Architecture
- **Language**: Rust (stable)
- **Framework**: Solana SDK
- **Network**: Solana Devnet
- **Structure**: Single binary application

## Features
- ✅ Connects to Solana devnet
- ✅ Generates new keypairs
- ✅ Checks cluster version
- ✅ Retrieves account balances
- ✅ Real-time blockchain interaction

## Dependencies
- `solana-program`: Core Solana program library
- `solana-client`: RPC client for blockchain interaction
- `solana-sdk`: Software development kit for Solana
- `serde`: Serialization framework
- `tokio`: Async runtime

## Recent Changes
- **2025-09-06**: Initial project setup with Rust and Solana dependencies
- **2025-09-06**: Created basic application connecting to Solana devnet
- **2025-09-06**: Configured workflow for automatic building and running

## Running the Application
The application runs automatically via the configured workflow. It will:
1. Connect to Solana devnet
2. Display cluster information
3. Generate a new keypair
4. Check account balance

## Next Steps
- Add smart contract functionality
- Implement transaction sending
- Add user interface components
- Deploy custom programs to blockchain