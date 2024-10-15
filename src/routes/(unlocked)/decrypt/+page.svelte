<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "$lib/components/ui/button";
    import Textarea from "@/components/ui/textarea/textarea.svelte";
    import { Input } from "@/components/ui/input";
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
    let decrypted_message = '';
    let password = '';
    let selected_priv_key = "";
    let selected_sign_key: string|undefined = undefined;
   
    async function decode() {
        try {
            let options = {
                pkeyId: selected_priv_key,
                message,
                passKey:password,
                signer:selected_sign_key
            };
            console.log(options);

            decrypted_message= await invoke('decrypt_message', options);
        } catch (err) {
            console.error(err);
            decrypted_message = '';
            toast.error(`Error decrypting message: ${err}`); 
        } 
    }
</script>

<div class="p-5 flex flex-col gap-2 w-full h-full">
    <div class="flex flex-row gap-2">
        <Select.Root selected={{value:"",label:""}} onSelectedChange={
            (selected) => {
                selected && (selected_priv_key = selected.value);
            }
        } >
            <Select.Trigger class="w-[180px]">
                <Select.Value placeholder="Private Key" />
            </Select.Trigger>
                <Select.Content>
                    {#each priv_keys as key}
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
                <Select.Value placeholder="Signer Public Key (Optional)" />
            </Select.Trigger >
                <Select.Content >
                    <Select.Item value={null}>None</Select.Item>
                    <Select.Item value={"#new"}>New</Select.Item>
                    {#each pub_keys as key}
                        <Select.Item value={key.key_id}>{key.nickname}</Select.Item>
                    {/each}
                </Select.Content>
        </Select.Root>
    </div>
    <Textarea class="w-full h-full" bind:value={message} placeholder="Paste Message Here" />
    <Input class="w-full" bind:value={password} placeholder="Password" />
    <Button on:click={() => decode()}>Decrypt</Button>
    <Textarea class="w-full h-full pointer-events-none" disabled bind:value={decrypted_message} placeholder="Decrypted Message" />
</div>

