use std::fmt;
use uuid::Timestamp;
use uuid::Uuid;

pub struct UUIDTime(Timestamp);

impl UUIDTime {
    fn to_unix_sec(&self) -> u64 {
        self.0.to_unix().0
    }
}

impl fmt::Display for UUIDTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_unix_sec().to_string())
    }
}

impl From<Uuid> for UUIDTime {
    fn from(id: Uuid) -> Self {
        UUIDTime(id.get_timestamp().unwrap())
    }
}
