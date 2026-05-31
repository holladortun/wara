#[derive(Debug, Clone)]
pub struct SshTarget {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub public_key: String,
    pub private_key: String,
    pub private_key_passphrase: Option<String>,
}

pub async fn run_controlled_commands(
    target: &SshTarget,
    commands: &[String],
) -> anyhow::Result<String> {
    tracing::info!(
        host = %target.host,
        port = target.port,
        username = %target.username,
        public_key_fingerprint = %fingerprint_public_key(&target.public_key),
        command_count = commands.len(),
        "planned controlled SSH command execution"
    );
    Ok(commands.join("\n"))
}

pub fn fingerprint_public_key(public_key: &str) -> String {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use sha2::{Digest, Sha256};

    let digest = Sha256::digest(public_key.trim().as_bytes());
    format!("SHA256:{}", STANDARD.encode(digest))
}
