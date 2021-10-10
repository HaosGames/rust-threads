use super::peer::Peer;
use crate::single_writer_log::{SingleWriterLog};
use ipfs_api::IpfsClient;
use std::error::Error;
use cid::Cid;
use std::io::ErrorKind;

pub struct Thread {
    // peers: std::collections::HashMap<String, Peer>
    log: SingleWriterLog,
    client: IpfsClient,
}

impl Thread {
    pub async fn new(body: String, client: IpfsClient) -> Thread {
        let log = SingleWriterLog::new(body, &client).await;
        Thread { log, client }
    }
    pub fn restore(head_node: String, client: IpfsClient) -> Thread {
        let log = SingleWriterLog::get(head_node);
        Thread {log, client}

    }
    pub fn forget() {}
    pub async fn add_entry(self, body: String) {
        self.log.append(body, &self.client).await;
    }
}
