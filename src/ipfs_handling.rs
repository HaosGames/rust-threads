use crate::node::Node;
use cid::multibase::Base;
use cid::multihash::{Code, MultihashGeneric};
use cid::{Cid, Version};
use ipfs_api::IpfsClient;
use std::io::{Cursor, ErrorKind};
use std::ops::Add;
use std::str::FromStr;
use futures::TryStreamExt;

impl Node {
    pub async fn add_to_ipfs(&self, client: &IpfsClient) -> Result<String, ErrorKind> {
        println!("Adding to IPFS...");
        let data = Cursor::new(self.encode_for_ipfs());
        match client.add(data).await {
            Ok(response) => {
                println!("Added to IPFS: {}", response.hash);
                Ok(response.hash)
            }
            Err(e) => {
                eprintln!("error adding file");
                Err(ErrorKind::Other)
            }
        }
    }

    pub fn to_cid_string(&self) -> String {
        let digest = serde_json::to_string(&self).unwrap();
        let input_digest = digest.as_bytes();
        let multi_hash = MultihashGeneric::wrap(12, input_digest).unwrap();
        let cid = Cid::new_v1(12, multi_hash);
        cid.to_string_of_base(Base::Base32Lower).unwrap()
    }

    pub async fn get_from_ipfs(hash: String, client: &IpfsClient) -> Result<Node, ErrorKind> {
        match client.object_get(&hash.as_str()).await {
            Ok(response) =>  {
                let node = Node::decode_from_ipfs(response.data);
                Ok(node)
            },
            Err(e) => {
                eprintln!("IPFS: Error getting file");
                Err(ErrorKind::Other)
            }
        }
    //     let result = client.cat(hash.as_str());
    }

    fn encode_for_ipfs (&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn decode_from_ipfs(data: String) -> Node {
        println!("{}", data);
        serde_json::from_slice(data.as_bytes()).unwrap()
        // serde_json::from_str(data.as_str()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ipfs_api::TryFromUri;

    #[tokio::test]
    async fn add_node_to_ipfs() {
        let client = IpfsClient::from_ipfs_config().unwrap();
        let node = Node::new_root(String::from("This is a test."));
        node.add_to_ipfs(&client).await;
    }

    #[tokio::test]
    async fn get_node_from_ipfs() {
        let client = IpfsClient::from_ipfs_config().unwrap();
        let node = Node::new_root(String::from("This is a test."));
        let data = serde_json::to_string(&node).unwrap();
        let data = Cursor::new(data);
        client.add(data).await.unwrap();
        let node = Node::get_from_ipfs(String::from(
            "QmbGXJcCcydmYp1Fkq3WabbTnQ1adTDJt3QfbgeZjhpMDc"
        ), &client).await.unwrap();
    }
}
