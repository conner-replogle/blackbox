import { Header } from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { usePrivateKeys } from "@/hooks/use-keys";
import { PlusCircleIcon, Trash, UserCircle } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { RemovePrivateKey } from "@/lib/api/pgp";
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
import { invoke } from "@tauri-apps/api/core";
import { Textarea } from "@/components/ui/textarea";


export default function Identities() {
  const [keys,reload] = usePrivateKeys();

  return (
    <section className="container h-screen flex flex-col">
      <Header />
      
      <ScrollArea className="h-[calc(100vh-100px)] pr-4">
        {keys.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
            <UserCircle size={40} className="mb-2" />
            <p>No identities found</p>
          </div>
        ) : (
          <div className="space-y-4">
            {keys.map((key) => (
              <Card key={key.nickname} className="group">
                <CardContent className="flex items-center justify-between p-4">
                  <div className="flex items-center gap-3">
                    <UserCircle className="text-muted-foreground" />
                    <span className="font-medium">{key.nickname}</span>
                  </div>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="opacity-0 group-hover:opacity-100 transition-opacity"
                    onClick={() => {RemovePrivateKey(key.key_id).then(()=>{reload()})}}
                  >
                    <Trash className="h-4 w-4 text-destructive" />
                  </Button>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </ScrollArea>
      <AddIdentity/>

      
    </section>
  );
}


function AddIdentity() {
  const {toast} = useToast();
  const [nickname,setNickname] = useState("");
  const [open,setOpen] = useState(false);
  const [description,setDescription] = useState("");
  const [privKey,setPrivKey] = useState("");

  const [pass,setPass] = useState("");

  return (
      <Dialog onOpenChange={setOpen} open={open}  >
        <DialogTrigger asChild>
          <button className="fixed bottom-6 right-6 p-4 rounded-full bg-primary-foreground text-primary-background">
            <PlusCircleIcon  size={24} />
          </button>
        </DialogTrigger>
        <DialogContent >
          <DialogHeader>
            <DialogTitle>Create Identity</DialogTitle>
            <DialogDescription>
              Save your private key into your identities database.
            </DialogDescription>
          </DialogHeader>
          <Input placeholder="Name" onChange={(a)=> setNickname(a.target.value)} />
          <Input placeholder="Description" onChange={(a)=> setDescription(a.target.value)}  />
          <Input placeholder="Password" type="password" onChange={(a)=> setPass(a.target.value)}  />
          <Textarea className="flex-grow h-full" onChange={(a)=> setPrivKey(a.target.value)} />
          <div className="flex gap-2">
            <Button 
             onClick={async ()=> {
              try{
              await SavePrivateKey(nickname,description,privKey,pass);
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
            }}
            
            className="w-full" variant={"default"} >Save</Button>
          </div>
        </DialogContent>
      </Dialog>    
  )
}