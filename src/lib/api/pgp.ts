import { invoke } from "@tauri-apps/api/core";
import { GeneratePGPKeysResponse } from "./types";



export async function GeneratePGPKeys(password?: string): Promise<GeneratePGPKeysResponse> {
    let out : GeneratePGPKeysResponse= await invoke("generate_key",{password: password ?? ""});
    return out;

}
    // pkey_id: &str,
    // message: &str,
    // pass_key: &str,
    // signer: Option<&str>,

export async function EncryptMessage(pkeyId:string,message:string,signer?:string, password?: string): Promise<string> {
    console.log(`PKey ${pkeyId}`)
    let out : string = await invoke("encrypt_message",{pkeyId,message,signer, passKey: password ?? ""});
    return out;

}

export async function DecryptMessage(pkeyId:string,message:string,signer?:string, password?: string): Promise<string> {
    console.log(`PKey ${pkeyId}`)
    let out : string = await invoke("decrypt_message",{pkeyId,message,passKey: password ?? "" ,signer });

    return out;

}

export async function SavePrivateKey(nickname: string, _description: string, privateKey: string,password: string): Promise<string> {

    return await invoke("add_private_key",{nickname,privateKey,password});
}
export async function SavePublicKey(nickname: string, _description: string, publicKey: string): Promise<string> {

    return await invoke("add_public_key",{nickname,publicKey});
}
export async function RemovePrivateKey(keyId: string): Promise<string> {
    return await invoke("remove_key",{keyId});
}

export async function RemovePublicKey(keyId: string): Promise<string> {
    return await invoke("remove_key",{keyId});
}
