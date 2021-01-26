use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Transaction {
}

/// Create digital signature of a transaction
pub fn sign(t: &Transaction, key: &Ed25519KeyPair) -> Signature {
    unimplemented!()
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &Transaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    unimplemented!()
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> Transaction {
        Default::default()
        //unimplemented!()
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }
}
