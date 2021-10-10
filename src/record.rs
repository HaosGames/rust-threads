use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Record {
    body: String,
    // header: Header,
}

#[derive(Serialize, Deserialize)]
struct Header {
    //time: Time,
//body_key: EncryptionKey,
}

impl Record {
    pub fn new(body: String) -> Record {
        Record { body }
    }
}
