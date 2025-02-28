use pqcrypto_kyber::kyber512::{keypair as kyber_keypair, encapsulate, decapsulate};
use pqcrypto_dilithium::dilithium2::{keypair as dilithium_keypair, sign, verify};

/// Generates a post-quantum key pair using Kyber512
pub fn generate_kyber_keypair() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = kyber_keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

/// Generates a Dilithium2 key pair
pub fn generate_dilithium_keypair() -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = dilithium_keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

/// Signs a message using Dilithium2
pub fn sign_message(message: &[u8], secret_key: &[u8]) -> Vec<u8> {
    sign(message, secret_key).as_bytes().to_vec()
}

/// Verifies a signature using Dilithium2
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    verify(signature, message, public_key).is_ok()
}
