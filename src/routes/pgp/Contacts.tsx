import { Header } from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { usePrivateKeys,usePublicKeys } from "@/hooks/use-keys";
import { CopyIcon, PlusCircleIcon, Trash, UserCircle } from "lucide-react";
import { ScrollArea } from "@/components/ui/scroll-area";
import { RemovePrivateKey, RemovePublicKey, SavePublicKey } from "@/lib/api/pgp";
import {  SavePrivateKey } from "@/lib/api/pgp";
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
import { Badge } from "@/components/ui/badge";


export default function Contacts() {
  const [publicKeys,reloadPub] = usePublicKeys();

  return (
    <section>
      <Header />
      
      <ScrollArea className="container h-[calc(100vh-100px)] pr-4">
        {publicKeys.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-40 text-muted-foreground">
            <UserCircle size={40} className="mb-2" />
            <p>No Contacts found</p>
          </div>
        ) : (
          <div className="space-y-4">
            {publicKeys.map((key) => (
              <Card key={key.nickname} className="group">
                <CardContent className="flex items-center justify-between p-4">
                  <div className="flex items-center gap-3">
                    <UserCircle className="text-muted-foreground" />
                    <span className="font-medium ">{key.nickname} </span>
                  </div>
                   <span className="flex justify-center items-center gap-3 h-8">
                      {key.is_me &&<Badge variant="outline">Identity</Badge>} 
                      <Button
                      variant="ghost"
                      size="icon"
                      className="hidden group-hover:flex transition-opacity"
                      onClick={() => {navigator.clipboard.writeText(key.public_key)}}
                      >
                        <CopyIcon className="h-4 w-4 " />
                      </Button>

                { !key.is_me && <Button
                    variant="ghost"
                    size="icon"
                    className="opacity-0 group-hover:opacity-100 transition-opacity"
                    onClick={() => {RemovePublicKey(key.key_id).then(()=>{reloadPub()})}}
                  >
                    <Trash className="h-4 w-4 text-destructive" />
                  </Button>}
                  </span>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </ScrollArea>
      <AddContact/>

      
    </section>
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
