extern crate portable_storage;
extern crate failure;
extern crate bytes;
extern crate serialization;

use std::io::Cursor;

use portable_storage::ser::{ToUnderlying, Error, invalid_storage_entry};
use portable_storage::StorageEntry;
use serialization::deserializer::{Deserialize, Deserializer, DeserializeBlob};
use serialization::serializer::{Serialize, Serializer};
use bytes::Buf;

/// H256 length in bytes.
pub const H256_LENGTH: usize = 32;

/// A 256-bit hash.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct H256(pub [u8; H256_LENGTH]);

impl H256 {
    pub fn new() -> H256 {
        H256::default()
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: &B) -> H256 {
        let bytes = bytes.as_ref();
        assert!(bytes.len() == H256_LENGTH, "invalid hash length");

        let mut h = Self::new();
        h.0.clone_from_slice(bytes);
        h
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<[u8; 32]> for H256 {
    fn from(v: [u8; 32]) -> H256 {
        H256(v)
    }
}

impl ToUnderlying for H256 {
    fn to_underlying(entry: &StorageEntry) -> Result<H256, Error> {
        match entry {
            &StorageEntry::Buf(ref v) => {
                // TODO: Add error handling, this panics on invalid slice length
                Ok(H256::from_bytes(v))
            }
            _ => Err(invalid_storage_entry("StorageEntry::Buf"))
        }
    }
}

impl From<H256> for StorageEntry {
    fn from(v: H256) -> StorageEntry {
        StorageEntry::Buf(v.as_bytes().to_vec())
    }
}

impl AsRef<[u8]> for H256 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl DeserializeBlob for H256 {
    fn deserialize_blob(v: &mut Cursor<&[u8]>) -> H256 {
        let mut hash = H256::new();
        v.copy_to_slice(&mut hash.0);
        hash
    }
}

impl Serialize for H256 {
    fn serialize<T: Serializer>(&self, serializer: &mut T) {
        serializer.serialize_blob(self)
    }
}

impl Deserialize for H256 {
    fn deserialize<'buf, T: Deserializer<'buf>>(deserializer: &'buf mut T) -> Self {
        deserializer.deserialize_blob()
    }
}
