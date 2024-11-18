import { Button} from "@/components/ui/button";
import { TextAreaWithCopy } from "@/components/mod-ui/copy-text-area";
import { GeneratePGPKeys } from "@/lib/api/pgp";
import { useState } from "react";
import { GeneratePGPKeysResponse } from "@/lib/api/types";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Input } from "@/components/ui/input";
import { useToast } from "@/hooks/use-toast";


export default function Generate() {
  const [loadingGeneration,setLoadingGeneration] = useState(false);
  const toast = useToast();
  const [keyPair,setKeyPair] = useState<GeneratePGPKeysResponse |undefined>(undefined);

  return (
    <section className="container flex flex-col h-full gap-2 p-5">

      <Button disabled={loadingGeneration} onClick={async ()=> {
        setLoadingGeneration(true);
        try{
          let out = await GeneratePGPKeys();
          setKeyPair(out);
        }catch(err){
          console.error(err);
          toast.toast({
            variant: "destructive",
            title: "Uh oh! Something went wrong.",
            description: err as string,
          })
        }

        setLoadingGeneration(false);

  

      }} >
       {loadingGeneration ? "Generating..." : "Generate Key Pair"} 
      </Button>

      <TextAreaWithCopy className="flex-grow h-full" value={keyPair?.private_key} disabled/>
      <TextAreaWithCopy className="flex-grow h-full"  value={keyPair?.public_key} disabled/>

      <Dialog >
        <DialogTrigger><Button className="w-full" variant={"secondary"}>Save Key Pair</Button></DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Save Key</DialogTitle>
            <DialogDescription>
              Save your key pair into your identities database.
            </DialogDescription>
          </DialogHeader>
          <Input placeholder="Name" />
          <Input placeholder="Description" />
          <div className="flex gap-2">
            <Button className="w-full" variant={"default"} onClick={()=> {

            }}>Save</Button>
          </div>
        </DialogContent>
      </Dialog>


    </section>
  );
}
