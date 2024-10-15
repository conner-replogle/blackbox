<script>
import "../app.css";
import { Toaster } from "$lib/components/ui/sonner";


</script>
<body class="dark text-foreground bg-background">
       <Toaster />

    <div class="flex flex-row h-screen w-screen ">
    <div class="w-[250px] border-r-2 border-primary bg-background p-5">
        <a href="/"><h1 class="text-primary font-bold text-4xl">BlackBox</h1></a>

        <nav class="pl-2 border-t-2 mt-5 gap-2">
            <a href="/privkey"><h2 class="text-primary text-sm">Add Private Key</h2></a>
            <a href="/pubkey"><h2 class="text-primary text-sm">Add Public Key</h2></a>

            <a href="/decrypt"><h2 class="text-primary text-sm">Decrypt Message</h2></a>
            <a href="/contacts"><h2 class="text-primary text-sm">Contact Book</h2></a>

            <a href="/identities"><h2 class="text-primary text-sm">Identities</h2></a>
        </nav>
    </div>
    <div class="w-full">
 
        
        <slot></slot>
    </div>
</div>
</body>
