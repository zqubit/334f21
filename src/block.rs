use serde::{Serialize, Deserialize};
use crate::crypto::hash::{H256, Hashable};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
}

impl Hashable for Block {
    fn hash(&self) -> H256 {
        unimplemented!()
    }
}

#[cfg(any(test, test_utilities))]
pub mod test {
    use super::*;
    use crate::crypto::hash::H256;

    pub fn generate_random_block(parent: &H256) -> Block {
        unimplemented!()
    }
}
