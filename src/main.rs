mod raft;

#[tokio::main]
async fn main() {
    let mut node = raft::node::RaftNode::new();
    node.run().await;
}

