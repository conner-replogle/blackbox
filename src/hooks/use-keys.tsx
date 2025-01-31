import { PrivateKey, PublicKey } from "@/lib/api/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export function usePrivateKeys(): [PrivateKey[],()=>void] {
    const [keys,setKeys] = useState<PrivateKey[]>([]);
    useEffect(()=>{
      // fetch keys
      reload();
      // setKeys
    },[]); 
    function reload(){
      invoke<PrivateKey[]>("get_private_keys").then((res)=>{
        setKeys(res);
      });
    }

    return [keys,reload];
}

export function usePublicKeys(): [PublicKey[],()=>void] {
    const [keys,setKeys] = useState<PublicKey[]>([]);
    useEffect(()=>{
      // fetch keys
      reload();
      // setKeys
    },[]); 
    function reload(){
      invoke<PublicKey[]>("get_public_keys").then((res)=>{
        setKeys(res);
      });
    }

    return [keys,reload];
}
