<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    import Textarea from '@/custom-components/TextArea2.svelte';
    import { Input } from "@/components/ui/input";
    import { toast } from "svelte-sonner";
	import { get_private_keys, get_public_keys } from "@/api";
	import { onMount } from "svelte";
    import * as Select from "$lib/components/ui/select";
    import type { Snapshot } from './$types';
    import { writable } from "svelte/store";
    import { browser } from "$app/environment";
    import { restorable } from "@/helper";
	
	
	
    
    type Output ={
        private_key:string,
        public_key:string
    }

    let passKey = '';
    let generating: boolean = false;

    //make the next few lines repeatable
    const out = restorable("generated_keys", {private_key:'', public_key:''});
    
    async function generate() {
        try {
            generating = true;

            out.set(await invoke('generate_key', {passKey}) as Output);
            
        } catch (err) {
            console.error(err);
            out.set( {
                private_key:'',
                public_key:''
            });
  
            toast.error(`Error generating key: ${err}`); 
        } finally {
            generating = false;
        }
    }
</script>

<div class="p-5 flex flex-col gap-2 w-full h-full">
    <Input placeholder="Name" />
    <Input placeholder="email" type="email" />
    <Input placeholder="Password" type="password" />
    <Button  disabled={generating} on:click={() => generate()}>{generating ? "Generating...":"Generate"}</Button>
    <Textarea class="w-full h-full" disabled value={$out.private_key} placeholder="Private Key" />
    <Textarea class="w-full h-full " disabled value={$out.public_key} placeholder="Public Key" />
    
  

</div>

