use crate::monero::{connect_to_rpc, MoneroWallet};
use monero_rpc::JsonTransaction;
use tauri::async_runtime::spawn_blocking;
use tauri::State;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn check_rpc(state: State<'_, MoneroWallet>) -> Result<bool, String> {
    let unlocked = state.read().await.is_some();
    log::debug!("Checking rpc: {}", unlocked);

    Ok(unlocked)
}

#[tauri::command]
pub async fn open_rpc(
    state: State<'_, MoneroWallet>,
    app: AppHandle,
    address: String,
) -> Result<bool, String> {
    log::debug!("Connecting");

    //attempting not to block thread maybe just async function does this but idc I already wrote this
    let pool = connect_to_rpc(&address).await.unwrap();
    log::debug!("Connected");
    state.write().await.replace(pool);
    Ok(true)
}

#[tauri::command]
pub async fn test_rpc(state: State<'_, MoneroWallet>) -> Result<(), String> {
    log::debug!("Testing");
    let out = state.read().await;
    let Some(rpc_client) = out.as_ref() else {
        return Err("Not Connected".to_string());
    };
    let tx_id = "7c50844eced8ab78a8f26a126fbc1f731134e0ae3e6f9ba0f205f98c1426ff60".to_string();
    let daemon_rpc_client = rpc_client.clone().daemon_rpc();
    let mut fixed_hash: [u8; 32] = [0; 32];
    hex::decode_to_slice(tx_id, &mut fixed_hash).unwrap();
    let tx = daemon_rpc_client
        .get_transactions(vec![fixed_hash.into()], Some(true), Some(true))
        .await;
    log::debug!("tx {:?}", tx);

    log::debug!(
        "unlock time: {:?}",
        serde_json::from_str::<JsonTransaction>(&tx.unwrap().txs_as_json.unwrap()[0])
    );

    Ok(())
}

#[tauri::command]
pub async fn close_rpc(state: State<'_, MoneroWallet>) -> Result<(), String> {
    log::debug!("Locking");

    state.write().await.take();

    Ok(())
}
