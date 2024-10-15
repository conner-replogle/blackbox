import { invoke } from "@tauri-apps/api/core";


export async function get_private_keys(): Promise<PrivateKey[]> {

    const response: PrivateKey[] = await invoke('get_private_keys');
    return response.map((key) => {
        return {
            key_id: key.key_id,
            nickname: key.nickname,
            private_key: key.private_key,
            created_at: new Date(key.created_at)
        }
    });

}


export async function get_public_keys(): Promise<PublicKey[]> {

    const response: PublicKey[] = await invoke('get_public_keys');
    return response.map((key) => {
        return {
            ...key,
            created_at: new Date(key.created_at)
        }
    });

}