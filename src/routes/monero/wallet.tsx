import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ConnectRPC } from "@/lib/api/monero";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";


export default function Wallet() {
  const [address,setAddress] = useState("");

  async function Connect(){
    await ConnectRPC(address);
    await invoke("test_rpc");
  }


  return (
    <section className="container">
      <Input placeholder="RPC Address" value={address} onChange={(e) => setAddress(e.target.value)}  />
      <Button onClick={Connect} >Connect To RPC</Button>
    </section>
  );
}
