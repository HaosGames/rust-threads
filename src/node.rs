use super::record::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cid::Cid;

#[derive(Serialize, Deserialize)]
pub struct Node {
    record: Record,
    previous_node: Option<String>,
}

impl Node {
    pub fn new_root(body: String) -> Node {
        let record = Record::new(body);
        Node {
            record,
            previous_node: None,
        }
    }

    pub fn new(body: String, previous_node: String) -> Node {
        let record = Record::new(body);
        Node {
            record,
            previous_node: Some(previous_node),
        }
    }
}
