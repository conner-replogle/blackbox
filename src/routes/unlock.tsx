import { Button, buttonVariants } from "@/components/ui/button";

import { Input } from "@/components/ui/input";
import { useState } from "react";
import { unlock } from "@/lib/api/database";
import { useToast } from "@/hooks/use-toast";
import { useNavigate } from "react-router-dom";
export default function UnlockPage() {
    const [password, setPassword] = useState("");
    const [unlocking, setUnlocking] = useState(false);
    const navigate = useNavigate();
    const { toast } = useToast()

    function Unlock(){
        setUnlocking(true);
        unlock(password).then((res)=>{
            console.log(res)
            setUnlocking(false);
            navigate("/");
        }).catch((err)=>{
            console.log(err)
            toast({
                variant: "destructive",
                title: "Uh oh! Something went wrong.",
                description: err,
            
              })
            setUnlocking(false);
        });
    } 


    function clickPress(event:any) {
      console.log(event.keyCode)
        if (event.keyCode == 13) {
            Unlock();
        }
    }
    
  return (
    <section className=" flex flex-col justify-center items-center h-screen gap-2">
    <img width={150}src="/static/blackbearlogo-clear.png"/>
    <Input className="w-min" onKeyDown={clickPress} onChange={(e)=> {setPassword(e.target.value)}}  type="password" placeholder="Password" />
    <Button disabled={unlocking} onClick={()=> {
        Unlock();
    }
    } >{unlocking ? "Unlocking...": "Unlock"}</Button>
    </section>
  );
}
