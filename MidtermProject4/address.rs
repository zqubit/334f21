use serde::{Serialize,Deserialize};

/// A 160-bit public address.
#[derive(Eq, PartialEq, Serialize, Deserialize, Clone, Hash, Default, Copy)]
pub struct H160([u8; 20]); 

impl std::fmt::Display for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = if let Some(precision) = f.precision() {
            if precision >= 40 {
                0
            } else {
                20 - precision / 2
            }
        } else {
            0
        };
        for byte_idx in start..20 {
            write!(f, "{:>02x}", &self.0[byte_idx])?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for H160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:>02x}{:>02x}..{:>02x}{:>02x}",
            &self.0[0], &self.0[1], &self.0[18], &self.0[19]
        )
    }
}

impl H160 {
    /// Create a new address from a public key.
    pub fn from_pubkey(pub_key_as_bytes: &[u8]) -> Self {
        let mut buffer: [u8; 20] = [0; 20];
        let digest = ring::digest::digest(&ring::digest::SHA256, pub_key_as_bytes);
        let digest_bytes = digest.as_ref();
        let last_20_bytes = &digest_bytes[digest_bytes.len() - 20..];
        assert_eq!(last_20_bytes.len(), 20);
        buffer[..].copy_from_slice(last_20_bytes);
        buffer.into()
    }
}

impl std::convert::AsRef<[u8]> for H160 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::convert::From<[u8; 20]> for H160 {
    fn from(input: [u8; 20]) -> H160 {
        H160(input)
    }
}
