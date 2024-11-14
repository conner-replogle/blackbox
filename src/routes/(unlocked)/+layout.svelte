<script lang="ts">

	import Button from "@/components/ui/button/button.svelte";
    import { listen } from '@tauri-apps/api/event';
    import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import { Separator } from "@/components/ui/separator";
    onMount(async () => {
        let auth = await invoke('check_auth')
        if (!auth){
            console.log("Not authenticated");
            goto("/");


        }
    });


</script>


<div class="flex flex-row h-screen w-screen ">
    <div class=" min-w-[150px] w-[150px] hidden md:flex border-r-2 border-primary bg-background p-3 items-center  h-screen  flex-col justify-between">
        <div class="flex flex-col items-center">
        <img alt="blackbearlogo" src="/blackbearlogo-clear.png" class="w-20 h-20"/>
        <Separator class="my-2"/>

        <nav class="flex flex-col pl-2 gap-2">
            <a href="/generate"><h2 class="text-primary text-sm">Generate Keys</h2></a>
            <a href="/privkey"><h2 class="text-primary text-sm">Add Private Key</h2></a>
            <a href="/pubkey"><h2 class="text-primary text-sm">Add Public Key</h2></a>

            <a href="/decrypt"><h2 class="text-primary text-sm">Decrypt Message</h2></a>
            <a href="/encrypt"><h2 class="text-primary text-sm">Encrypt Message</h2></a>
            <a href="/contacts"><h2 class="text-primary text-sm">Contact Book</h2></a>

            <a href="/identities"><h2 class="text-primary text-sm">Identities</h2></a>
        </nav>
        </div>
        <div class="w-full">
        <Separator class="mb-2 "/>

        <Button class="w-full" on:click={async()=> {await invoke("lock");goto("/");}}>
            Lock
        </Button>
        </div>
    </div>
    <div class=" min-w-[150px] w-[150px]  md:hidden border-r-2 border-primary bg-background p-3 items-center  h-screen  flex-col justify-between">
        <div class="flex flex-col items-center">
        <img alt="blackbearlogo" src="/blackbearlogo-clear.png" class="w-20 h-20"/>
        <Separator class="my-2"/>

        <nav class="flex flex-col pl-2 gap-2">
            <a href="/generate"><h2 class="text-primary text-sm">Generate Keys</h2></a>
            <a href="/privkey"><h2 class="text-primary text-sm">Add Private Key</h2></a>
            <a href="/pubkey"><h2 class="text-primary text-sm">Add Public Key</h2></a>

            <a href="/decrypt"><h2 class="text-primary text-sm">Decrypt Message</h2></a>
            <a href="/encrypt"><h2 class="text-primary text-sm">Encrypt Message</h2></a>
            <a href="/contacts"><h2 class="text-primary text-sm">Contact Book</h2></a>

            <a href="/identities"><h2 class="text-primary text-sm">Identities</h2></a>
        </nav>
        </div>
        <div class="w-full">
        <Separator class="mb-2 "/>

        <Button class="w-full" on:click={async()=> {await invoke("lock");goto("/");}}>
            Lock
        </Button>
        </div>
    </div>
    <div class="w-full">
 
        
        <slot></slot>
    </div>
</div>

