// Copyright 2025 stellar-sim contributors
// SPDX-License-Identifier: Apache-2.0

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stellar-sim", about = "A local Stellar/Soroban transaction simulator", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simulate a Soroban contract invocation locally
    Simulate {
        /// Path to the WASM contract file
        #[arg(long)]
        wasm: String,
        /// Function name to invoke
        #[arg(long)]
        function: String,
        /// JSON-encoded arguments array
        #[arg(long, default_value = "[]")]
        args: String,
        /// Path to a ledger snapshot JSON file
        #[arg(long)]
        ledger: Option<String>,
        /// Output results as JSON
        #[arg(long)]
        json: bool,
    },
    /// Decode and display diagnostic events from base64 XDR
    Events {
        /// Base64-encoded XDR transaction result
        #[arg(long)]
        tx: String,
    },
    /// Inspect a WASM contract file
    Inspect {
        /// Path to the WASM file
        #[arg(long)]
        wasm: String,
    },
    /// Capture ledger state for a contract to a JSON snapshot
    Snapshot {
        /// Stellar RPC endpoint URL
        #[arg(long)]
        rpc: String,
        /// Contract ID to snapshot
        #[arg(long)]
        contract: String,
        /// Output file path
        #[arg(long, default_value = "./snapshot.json")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Simulate { wasm, function, args, ledger, json } => {
            println!("Simulating {} in {}", function, wasm);
            let _ = (args, ledger, json);
        }
        Commands::Events { tx } => {
            println!("Decoding events from tx: {}", tx);
        }
        Commands::Inspect { wasm } => {
            println!("Inspecting WASM: {}", wasm);
        }
        Commands::Snapshot { rpc, contract, output } => {
            println!("Snapshotting contract {} from {} to {}", contract, rpc, output);
        }
    }
}
