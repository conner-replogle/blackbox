<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
import Textarea from '@/custom-components/TextArea2.svelte';    import { Input } from "@/components/ui/input";
    import { toast } from "svelte-sonner";

  
    let nickname = "";
    let publicKey = "";
  
    async function add_key() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

      try{
      const id:string = await invoke("add_public_key", { nickname,publicKey });
      toast.success(`Public key added ${id} successfully`);
      }catch (err) {
        console.log(err);
        toast.error(`Error adding private key: ${err}`);
      }
    }
  </script>
  
  <div class="p-5 flex flex-col gap-2 w-full h-full">
    <p class="text-primary text-2xl">Add Public Key</p>

    <Input class="w-full" bind:value={nickname} placeholder="Nickname" />
    <Input class="w-full" bind:value={nickname} placeholder="Name" />
    <Input class="w-full" bind:value={nickname} placeholder="Location" />
    
    <Textarea class="w-full h-full" bind:value={publicKey} placeholder="Paste Public Key Here" />
    <Button class="w-full" on:click={add_key}>Add</Button>
   
  </div>
  
  