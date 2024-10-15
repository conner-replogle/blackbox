<script>
	import { goto } from "$app/navigation";
    import * as Card from "$lib/components/ui/card";
	import Button from "@/components/ui/button/button.svelte";
	import Input from "@/components/ui/input/input.svelte";
	import Label from "@/components/ui/label/label.svelte";
	import { invoke } from "@tauri-apps/api/core";

    let password = '';
    async function unlock(){
        let outcome = await invoke('unlock', {password});
        if(outcome){
            console.log("REdirecting to /");
            goto("/");
        }
    }
</script>

<div class="flex flex-col w-full h-full justify-center items-center">
    <h1 class="text-4xl font-bold mb-8">BlackBox</h1>
    <Card.Root>
        <Card.Header>
            <Card.Title>Unlock</Card.Title>

        </Card.Header>
        <Card.Content>
            <div class="flex flex-col space-y-2">
               
                <Label for="password">Password</Label>
                <Input bind:value={password} type="password" id="password" name="password" />
            </div>
        </Card.Content>
        <Card.Footer class="gap-2">
            <Button on:click={unlock}>Unlock</Button>
            <Button href="/forgot-password">Forgot Password</Button>
        </Card.Footer>

    </Card.Root>
</div>