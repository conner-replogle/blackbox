
import { invoke } from "@tauri-apps/api/core";



export async function ConnectRPC(address: string): Promise<string> {
    return await invoke("open_rpc",{address});
}
