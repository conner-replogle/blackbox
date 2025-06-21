use std::sync::{Arc};

use monero_rpc::{RpcClient, RpcClientBuilder};
use tokio::sync::RwLock;




pub type MoneroWallet = Arc<RwLock<Option<RpcClient>>>;








pub async fn connect_to_rpc(address: &str) -> anyhow::Result<RpcClient>{
    RpcClientBuilder::new().build(address)
}
