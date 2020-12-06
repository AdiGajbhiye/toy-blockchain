use crypto_hash::{digest, Algorithm};

pub type Bytes = Vec<u8>;

pub trait Hashable {
    fn bytes(&self) -> Bytes;

    fn hash(&self) -> Bytes {
        digest(Algorithm::SHA256, &self.bytes())
    }
}
