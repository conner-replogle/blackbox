import { Key } from "@/lib/api/types";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export function usePrivateKeys(): [Key[],()=>void, boolean] {
    const [keys,setKeys] = useState<Key[]>([]);
    const [loading,setLoading] = useState(true);
    useEffect(()=>{
      // fetch keys
      reload();
      // setKeys
    },[]); 
    function reload(){
      setLoading(true);
      invoke<Key[]>("get_private_keys").then((res)=>{
        setKeys(res);
        setLoading(false);
      });
    }

    return [keys,reload,loading];
}

export function usePublicKeys(): [Key[],()=>void, boolean] {
    const [keys,setKeys] = useState<Key[]>([]);
    const [loading,setLoading] = useState(true);
    useEffect(()=>{
      // fetch keys
      reload();
      // setKeys
    },[]); 
    function reload(){
      setLoading(true);
      invoke<Key[]>("get_public_keys").then((res)=>{
        setKeys(res);
        setLoading(false);
      });
    }

    return [keys,reload,loading];
}
