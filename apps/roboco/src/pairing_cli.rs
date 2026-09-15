//! Pairing administration opens SQLite directly and never takes the engine lock.
use clap::Subcommand;
use roboco_engine::pairing::{DEFAULT_TTL_SECONDS, PairingStore, pairing_url};

#[derive(Subcommand)]
pub enum EngineCommand {
    /// Manage credentials for remote clients.
    Pairing {
        #[command(subcommand)]
        command: PairingCommand,
    },
}

#[derive(Subcommand)]
pub enum PairingCommand {
    /// Mint a short-lived, single-use pairing URL.
    Create {
        /// HTTP(S) address clients use to reach this engine.
        #[arg(long)]
        base_url: String,
        #[arg(long, default_value = "")]
        label: String,
        #[arg(long, default_value_t = DEFAULT_TTL_SECONDS)]
        ttl_seconds: u64,
        #[arg(long)]
        json: bool,
    },
    /// List paired sessions, including revocation and last-seen timestamps.
    List {
        #[arg(long)]
        json: bool,
    },
    /// Revoke a session immediately for subsequent authentication attempts.
    Revoke { session_id: String },
}

pub fn run(command: EngineCommand, data_dir: &std::path::Path) -> anyhow::Result<()> {
    let EngineCommand::Pairing { command } = command;
    let store = PairingStore::open(data_dir)?;
    match command {
        PairingCommand::Create {
            base_url,
            label,
            ttl_seconds,
            json,
        } => {
            // Validate the address before issuing a credential.
            pairing_url(&base_url, "")?;
            let code = store.create_code(&label, ttl_seconds)?;
            let url = pairing_url(&base_url, &code.credential)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({"id":code.id,"url":url,"expiresAt":code.expires_at})
                );
            } else {
                println!("{url}");
                println!(
                    "Expires at {} (Unix milliseconds). Use this link once to pair a client.",
                    code.expires_at
                );
            }
        }
        PairingCommand::List { json } => {
            let sessions = store.list_sessions()?;
            if json {
                println!("{}", serde_json::to_string(&sessions)?);
            } else if sessions.is_empty() {
                println!("No paired sessions.");
            } else {
                for session in sessions {
                    println!(
                        "{}\t{}\tlast seen {}\t{}",
                        session.id,
                        session.label,
                        session.last_seen,
                        if session.revoked_at.is_some() {
                            "revoked"
                        } else {
                            "active"
                        }
                    );
                }
            }
        }
        PairingCommand::Revoke { session_id } => {
            anyhow::ensure!(
                store.revoke(&session_id)?,
                "active session not found: {session_id}"
            );
            println!("Revoked {session_id}");
        }
    }
    Ok(())
}
