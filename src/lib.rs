use futures::executor::block_on;

mod node;
mod peer;
mod record;
mod single_writer_log;
mod thread;
mod ipfs_handling;
mod error;

#[cfg(test)]
mod tests {
    use super::*;
    use ipfs_api::{IpfsClient, TryFromUri};
    use crate::thread::Thread;

    #[tokio::test]
    async fn add_entry_to_thread() {
        // assert_eq!(2 + 2, 4);
        let client = IpfsClient::from_ipfs_config().unwrap();
        let mut thread = Thread::new(String::from("This is a test."), client).await;

    }


}
