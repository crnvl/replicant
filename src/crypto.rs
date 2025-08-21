use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use rand::TryRngCore;
use rand::rngs::OsRng;

pub fn generate_token() -> String {
    let mut buf = [0u8; 192];
    OsRng.try_fill_bytes(&mut buf).ok();
    STANDARD.encode(&buf)
}
