use crate::node::Node;
use crate::record::Record;
use cid::multibase::Base;
use cid::multihash::{MultihashGeneric, Sha2_256};
use cid::{Version, CidGeneric, Cid};
use ipfs_api::IpfsClient;
use std::error::Error;
use std::str::FromStr;

pub struct SingleWriterLog {
    head: String
}

impl SingleWriterLog {
    pub async fn new(body: String, client: &IpfsClient) -> SingleWriterLog {
        let node = Node::new_root(body);
        let node_cid = node.add_to_ipfs(&client).await.unwrap();
        SingleWriterLog {
            head: node_cid,
        }
    }
    pub fn get(head: String) -> SingleWriterLog {
        SingleWriterLog { head }
    }
    pub async fn append(mut self, body: String, client: &IpfsClient) {
        let new_node = Node::new(body, self.head);
        let hash = new_node.add_to_ipfs(client).await.unwrap();
        self.head = hash;
    }
}
