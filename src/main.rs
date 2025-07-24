mod store;
mod leader;
mod proxy;

use tokio::join;

#[tokio::main]
async fn main() {
    join!(
        leader::start_leader_server(),
        proxy::start_proxy(4000),
        proxy::start_proxy(4001)
    );
}

