import { Button} from "@/components/ui/button";
import { TextAreaWithCopy } from "@/components/mod-ui/copy-text-area";
import { GeneratePGPKeys, SavePrivateKey } from "@/lib/api/pgp";
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
  const [password,setPassword] = useState("");


  return (

    <div className="container flex flex-col h-full gap-2 ">
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
    <Input placeholder="Password" type="password" onChange={(a)=> {setPassword(a.target.value)}} />
    <TextAreaWithCopy className="flex-grow h-full"  value={keyPair?.public_key} disabled/>

    <SaveKeyPair private_key={keyPair?.private_key} password={password} />  
    </div>
  );
}


function SaveKeyPair({private_key,password}: {private_key?: string,password: string}) {
  const {toast} = useToast();
  const [nickname,setNickname] = useState("");
  const [open,setOpen] = useState(false);
  const [description,setDescription] = useState("");
  return (
      <Dialog onOpenChange={setOpen} open={open}  >
        <DialogTrigger asChild><Button disabled={private_key == null} className="w-full" variant={"secondary"}>Save Key Pair</Button></DialogTrigger>
        <DialogContent >
          <DialogHeader>
            <DialogTitle>Save Key</DialogTitle>
            <DialogDescription>
              Save your key pair into your identities database.
            </DialogDescription>
          </DialogHeader>
          <Input placeholder="Name" onChange={(a)=> setNickname(a.target.value)} />
          <Input placeholder="Description" onChange={(a)=> setDescription(a.target.value)}  />
          <div className="flex gap-2">
            <Button className="w-full" variant={"default"} onClick={async ()=> {
              try{
              await SavePrivateKey(nickname,description,private_key!,password);
              setOpen(false);
              toast({
                title: "Private Key Succefssfully Saved",
              })
              }catch(err){
                console.error(err);
                toast({
                  variant: "destructive",
                  title: "Uh oh! Something went wrong.",
                  description: err as string,
                })
              }
            }}>Save</Button>
          </div>
        </DialogContent>
      </Dialog>    
  )
}
