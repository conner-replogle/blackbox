<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
import Textarea from '@/custom-components/TextArea2.svelte';   
    import { toast } from "svelte-sonner";
	import { get_private_keys, get_public_keys } from "@/api";
	import { onMount } from "svelte";
    import * as Select from "$lib/components/ui/select";

    let priv_keys:PrivateKey[] = []; 
    let pub_keys:PublicKey[] = [];
    async function fetchData() {
        try {
            priv_keys = await get_private_keys();
      
            pub_keys = await get_public_keys();
        } catch (err) {
            toast.error(`Error getting private keys: ${err}`); 
        }    
    }

    onMount(fetchData)

    let message = '';
    
    let encrypted_message = '';
    let selected_priv_key = "";
    let selected_sign_key: string|undefined = undefined;
    let passKey = '';
    async function decode() {
        try {
            let options = {
                pkeyId: selected_priv_key,
                message,
                passKey,
                signer:selected_sign_key
            };
            console.log(options);

            encrypted_message= await invoke('encrypt_message', options);
        } catch (err) {
            console.error(err);
            encrypted_message = '';
            toast.error(`Error encrypting message: ${err}`); 
        } 
    }
</script>

<div class="p-5 flex flex-col gap-2 w-full h-full">
    <div class="flex flex-row gap-2 ">
        <Select.Root selected={{value:"",label:""}} onSelectedChange={
            (selected) => {
                selected && (selected_priv_key = selected.value);
            }
        } >
            <Select.Trigger class="w-[180px]">
                <Select.Value placeholder="Public Key" />
            </Select.Trigger>
                <Select.Content>
                    {#each pub_keys as key}
                    <Select.Item value={key.key_id}>{key.nickname}</Select.Item>
                    {/each}
            </Select.Content>
        </Select.Root>
        <Select.Root selected={{value:"",label:""}}  onSelectedChange={
            (selected) => {
                selected && (selected_sign_key = selected.value);            }
        }
        >
            <Select.Trigger class="w-[180px]">
                <Select.Value placeholder="Signer Private Key (Optional)" />
            </Select.Trigger >
                <Select.Content >
                    <Select.Item value={null}>None</Select.Item>
                    <Select.Item value={"#new"}>New</Select.Item>
                    {#each priv_keys as key}
                        <Select.Item value={key.key_id}>{key.nickname}</Select.Item>
                    {/each}
                </Select.Content>
        </Select.Root>
    </div>
    <Textarea class="w-full h-full flex-grow" bind:value={message} placeholder="Paste Encrypted Message Here" />
    <Button on:click={() => decode()}>Encrypt</Button>
    <Textarea class="w-full h-full flex-grow" disabled bind:value={encrypted_message} placeholder="Decrypted Message" />
</div>

