use serde::{Serialize, Serializer};
use std::convert::Into;
use std::fmt;
use std::ops::Deref;

pub struct DiscordId([u8; 8]);

impl Serialize for DiscordId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.into())
    }
}

impl Deref for DiscordId {
    type Target = [u8; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for DiscordId {
    fn from(id: u64) -> Self {
        DiscordId(id.to_be_bytes())
    }
}

impl From<DiscordId> for u64 {
    fn from(id: DiscordId) -> u64 {
        u64::from_be_bytes(*id)
    }
}

impl From<&DiscordId> for u64 {
    fn from(id: &DiscordId) -> u64 {
        u64::from_be_bytes(**id)
    }
}

impl From<Vec<u8>> for DiscordId {
    fn from(vec: Vec<u8>) -> Self {
        DiscordId(vec.try_into().unwrap())
    }
}

impl fmt::Display for DiscordId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id: u64 = self.into();
        f.write_str(&id.to_string())
    }
}
