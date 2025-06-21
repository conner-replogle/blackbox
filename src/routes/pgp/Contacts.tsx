import { Header } from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { CopyIcon, PlusCircleIcon, Trash, UserCircle } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";

import { useState } from "react";
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
import { Textarea } from "@/components/ui/textarea";
import {  RemovePublicKey, SavePublicKey } from "@/lib/api/pgp";

import { usePublicKeys } from "@/hooks/use-keys";
import { Identity } from "@/components/organisms/Identity";


export default function Contacts() {
  const [publicKeys,reloadPub,loading] = usePublicKeys();

  return (
    <div>

      <ScrollArea className="container h-[calc(100vh-100px)] px-4">
        {publicKeys.length === 0  && !loading? (
          <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
            <UserCircle size={40} className="mb-2" />
            <p>No Contacts found</p>
          </div>
        ) : (
          <div className="space-y-4">
            {publicKeys.map((key) => (
              <Identity Key={key} onDelete={() => {RemovePublicKey(key.key_id).then(()=>{reloadPub()});}}/>
             
            ))}
          </div>
        )}
      </ScrollArea>
      <AddContact/>

      
    </div>
  );
}


function AddContact() {
  const {toast} = useToast();
  const [nickname,setNickname] = useState("");
  const [open,setOpen] = useState(false);
  const [description,setDescription] = useState("");
  const [pubKey,setPubKey] = useState("");


  return (
      <Dialog onOpenChange={setOpen} open={open}  >
        <DialogTrigger asChild>
          <button className="fixed bottom-6 right-6 p-4 rounded-full bg-primary-foreground text-primary-background">
            <PlusCircleIcon  size={24} />
          </button>
        </DialogTrigger>
        <DialogContent >
          <DialogHeader>
            <DialogTitle>Create Contact</DialogTitle>
            <DialogDescription>
              Save your public key into your contacts.
            </DialogDescription>
          </DialogHeader>
          <Input placeholder="Name" onChange={(a)=> setNickname(a.target.value)} />
          <Input placeholder="Description" onChange={(a)=> setDescription(a.target.value)}  />
          <Textarea className="flex-grow h-full" onChange={(a)=> setPubKey(a.target.value)} />
          <div className="flex gap-2">
            <Button 
             onClick={async ()=> {
              try{
              await SavePublicKey(nickname,description,pubKey);
              setOpen(false);
              toast({
                title: "Public Key Succefssfully Saved",
              })
              }catch(err){
                console.error(err);
                toast({
                  variant: "destructive",
                  title: "Uh oh! Something went wrong.",
                  description: err as string,
                })
              }
            }}
            
            className="w-full" variant={"default"} >Save</Button>
          </div>
        </DialogContent>
      </Dialog>    
  )
}
