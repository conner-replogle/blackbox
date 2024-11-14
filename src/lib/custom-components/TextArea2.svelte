<script>
    export let value = "";
    import {Textarea} from "$lib/components/ui/textarea";
    import Button from "@/components/ui/button/button.svelte";
    import { ClipboardIcon, CopyIcon, LucideClipboard } from "lucide-svelte";
    // Text Area compoenent with a clipboard in top right corner to copy text
   
</script>


<div class={$$restProps.class} >
    <Textarea {...$$restProps} bind:value={value} />
    <Button 
        on:click={() => navigator.clipboard.writeText(value)} 
        class="w-8 h-8 p-0 hover:bg-secondary bg-background absolute bottom-1 right-5"
    >
        <CopyIcon strokeWidth={3}  class="stroke-primary" size={18}/>
    </Button>
</div>

