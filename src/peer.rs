use super::single_writer_log::SingleWriterLog;
use ipfs_api::IpfsClient;

pub struct Peer {
    id: String,
    log: SingleWriterLog,
}

impl Peer {
    pub async fn new(id: String, body: String, client: &IpfsClient) -> Peer {
        let log = SingleWriterLog::new(body, &client).await;
        Peer { id, log }
    }
}
