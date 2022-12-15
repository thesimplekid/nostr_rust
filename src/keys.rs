use rand::rngs::OsRng;
// use secp256k1::{PublicKey, SecretKey, SECP256K1};
use k256::schnorr::{signature::DigestVerifier, Signature, SigningKey, VerifyingKey};

// TODO: implement bech32 keys

/// Get a random secret key
/// # Example
/// ```
/// use nostr_rust::keys::get_random_secret_key;
/// let (secret_key, public_key) = get_random_secret_key();
/// ```
pub fn get_random_secret_key() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key.clone(), *verifying_key)
}

/// Get a secret key from a hex string
/// # Example
/// ```rust
/// use nostr_rust::keys::secret_key_from_str;
/// let secret_key = secret_key_from_str(env!("SECRET_KEY"));
/// assert!(secret_key.is_ok());
/// ```
pub fn secret_key_from_str(s: &str) -> Result<SigningKey, String> {
    let decoded_hex = &hex::decode(s);
    match decoded_hex {
        Ok(decoded_hex) => match SigningKey::from_bytes(decoded_hex) {
            Ok(secret_key) => Ok(secret_key),
            Err(_) => Err("Invalid secret key".to_string()),
        },
        Err(_) => Err("Invalid hex format".to_string()),
    }
}

/// Get a public key from a secret key
/// # Example
/// ```rust
/// use nostr_rust::keys::{secret_key_from_str, get_public_key_from_secret};
///
/// let secret_key = secret_key_from_str(env!("SECRET_KEY")).unwrap();
/// let public_key = get_public_key_from_secret(&secret_key);
/// ```
pub fn get_public_key_from_secret(secret_key: &SigningKey) -> VerifyingKey {
    *secret_key.verifying_key()
}

/// Returns Verifying key given hex verifying key
pub fn verifying_key_from_hex(verifying_key_hex: &str) -> VerifyingKey {
    let h = hex::decode(verifying_key_hex).unwrap();
    VerifyingKey::from_bytes(&h).unwrap()
}

/// Returns signature from hex ecoded string
pub fn signature_from_hex(signature: &str) -> Signature {
    let sig_hex = hex::decode(signature).unwrap();
    Signature::try_from(sig_hex.as_ref()).unwrap()
}

/// Generate a hex secret key and a hex public key from a secret key
/// # Example
/// ```rust
/// use nostr_rust::keys::{secret_key_from_str, get_str_keys_from_secret};
///
/// let secret_key = secret_key_from_str(env!("SECRET_KEY")).unwrap();
/// let (secret_key_str, public_key_str) = get_str_keys_from_secret(&secret_key);
///
/// assert_eq!(secret_key_str, env!("SECRET_KEY"));
/// assert_eq!(public_key_str, env!("PUBLIC_KEY"));
/// ```
pub fn get_str_keys_from_secret(secret_key: &SigningKey) -> (String, String) {
    (
        hex::encode(&secret_key.to_bytes()),
        hex::encode(&secret_key.verifying_key().to_bytes()),
    )
}

/// Normalize a public key
pub fn normalize_public_key(public_key: &str) -> String {
    public_key.to_string()[2..].to_string()
}
