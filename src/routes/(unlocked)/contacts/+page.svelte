<script lang="ts">
    import { onMount } from 'svelte';
  
    import { Button } from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card";
      import { MessageSquareText } from 'lucide-svelte';
      import { get_private_keys, get_public_keys } from '@/api';
      import Textarea from '@/custom-components/TextArea2.svelte';
      import { invoke } from '@tauri-apps/api/core';
      import { toast } from 'svelte-sonner';
  
  
    let data:PublicKey[] = [];
    let loading = true;
    let error = '';
  
  
  
    async function fetchData() {
      try {
  
        data = await get_public_keys();
      } catch (err) {
        toast.error(`Error getting public keys: ${err}`); 
      } finally {
        loading = false;
      }
  
    }
  
    onMount(fetchData)
  </script>
  
  <div class="container">
  
  
  
  
    {#if loading}
      <p>Loading...</p>
    {:else if error}
      <p>Error: {error}</p>
    {:else}
  
  
   
      <div class="flex flex-col p-5">
        {#each data as key}
       
        <Card.Root >
          <div class="flex flex-row justify-between items-center px-5 py-3">
          <div class="flex flex-col justify-center">
            <Card.Title>{key.nickname}</Card.Title>
            <Card.Description>
              {key.created_at.toDateString()}
            </Card.Description>
          </div>
          <Button size={"icon"}><MessageSquareText /></Button>
          </div>
          <!-- <Card.Content>
            <p>{key.privkey.slice(100,2)}</p>
          </Card.Content> -->
     
        </Card.Root>
          
        {/each}
      </div>
    {/if}
  
  </div>
  
  