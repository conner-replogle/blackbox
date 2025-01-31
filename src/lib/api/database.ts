import { invoke } from "@tauri-apps/api/core";

export async function unlock(password: string){
    let out = await invoke("unlock",{password: password});
    console.log(out);
}

export async function lock(){
  await invoke("lock");
}

export async function check_status(): Promise<boolean>{
    return await invoke("check_auth",{})
}
