import { invoke } from "@tauri-apps/api/core";
import { GeneratePGPKeysResponse } from "./types";



export async function GeneratePGPKeys(password?: string): Promise<GeneratePGPKeysResponse> {
    let out : GeneratePGPKeysResponse= await invoke("generate_key",{password: password ?? ""});
    return out;

}


export async function SaveKeys(name: string, description: string, public_key: string, private_key: string) {

    
    await invoke("save_key",{name,description,public_key,private_key});
}