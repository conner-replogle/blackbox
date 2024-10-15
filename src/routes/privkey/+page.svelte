<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    import Textarea from "@/components/ui/textarea/textarea.svelte";
    import { Input } from "@/components/ui/input";
    import { toast } from "svelte-sonner";

  
    let nickname = "";
    let privateKey = "";
  
    async function add_key() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

      try{
      let id:string = await invoke("add_private_key", { nickname,privateKey });
      toast.success(`Private key added ${id} successfully`);
      }catch (err) {
        console.log(err);
        toast.error(`Error adding private key: ${err}`);
      }
    }
  </script>
  
  <div class="p-5 flex flex-col gap-2 w-full h-full">
    <p class="text-primary text-2xl">Add Private Key</p>

    <Input class="w-full" bind:value={nickname} placeholder="Nickname" />
    
    <Textarea class="w-full h-full" bind:value={privateKey} placeholder="Paste Private Key Here" />
    <Button class="w-full" on:click={add_key}>Add</Button>
   
  </div>
  
  