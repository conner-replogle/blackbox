<script lang="ts">
	import { Toaster } from "@/components/ui/sonner";
    import "../app.css"
   
</script>

<body class="dark text-foreground bg-background w-screen h-screen m-0 overflow-hidden">
    <Toaster />
    <slot/>
</body> 