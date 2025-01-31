import { TextAreaWithCopy } from "@/components/mod-ui/copy-text-area";
import { PrivateKey, PublicKey } from "@/lib/api/types";
import { useState } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { usePrivateKeys, usePublicKeys } from "@/hooks/use-keys";
import { EncryptMessage, DecryptMessage } from "@/lib/api/pgp";
import { useToast } from "@/hooks/use-toast";
import { Button } from "@/components/ui/button";
import { Header } from "@/components/header";
import { Input } from "@/components/ui/input";
export default function Message() {
  const { toast } = useToast();
  const [keys] = usePrivateKeys();
  const [pubKeys] = usePublicKeys();
  const [message, setMessage] = useState("");
  const [outputMessage, setOutputMessage] = useState("");
  const [pKey, setPKey] = useState("");
  const [signKey, setSignKey] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);

  async function encryptMessage() {
    setLoading(true);
    try {
      let out = await EncryptMessage(pKey, message, signKey.length == 0 ? undefined : signKey, password);
      setOutputMessage(out);
    } catch (err) {
      console.error(err);
      toast({
        variant: "destructive",
        title: "Uh oh! Something went wrong.",
        description: err as string,
      });
    }
    setLoading(false);
  }

  async function decryptMessage() {
    setLoading(true);
    try {
      let out = await DecryptMessage(pKey, message, signKey.length == 0? undefined : signKey,password);
      setOutputMessage(out);
    } catch (err) {
      console.error(err);
      toast({
        variant: "destructive",
        title: "Uh oh! Something went wrong.",
        description: err as string,
      });
    }
    setLoading(false);
  }

  function renderEncryptTab() {
    return (
      <TabsContent value="encrypt" className="w-full !mt-0 flex flex-col gap-3">
        <div className="flex gap-3 flex-row justify-center w-full mt-2">
          <Select onValueChange={(a) => setPKey(a)}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Public Key" />
            </SelectTrigger>
            <SelectContent>
              {pubKeys.map((key: PublicKey) => (
                <SelectItem key={key.key_id} value={key.key_id}>
                  {key.nickname}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          <Select onValueChange={(a) => setSignKey(a)}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Signing Key" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={null as unknown as string}>None</SelectItem>
              {keys.map((key: PrivateKey) => (
                <SelectItem key={key.key_id} value={key.key_id}>
                  {key.nickname}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <Input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} className="flex-grow"/>
        <TextAreaWithCopy className="flex-grow h-full" value={message} onChange={(e) => setMessage(e.target.value)} />
        <Button className="w-full" onClick={() => encryptMessage()}>
          Cook
        </Button>
        <TextAreaWithCopy value={outputMessage} disabled className="flex-grow h-full" />
      </TabsContent>
    );
  }

  function renderDecryptTab() {
    return (
      <TabsContent value="decrypt" className="w-full flex flex-col !mt-0  gap-3">
        <div className="flex gap-3 flex-row justify-center w-full mt-2">
          <Select onValueChange={(a) => setPKey(a)}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Private Key" />
            </SelectTrigger>
            <SelectContent>
              {keys.map((key: PrivateKey) => (
                <SelectItem key={key.key_id} value={key.key_id}>
                  {key.nickname}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
         <Select onValueChange={(a) => setSignKey(a)}>
            <SelectTrigger className="w-full">
              <SelectValue placeholder="Signing Key" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value={null as unknown as string}>None</SelectItem>
              {pubKeys.map((key: PublicKey) => (
                <SelectItem key={key.key_id} value={key.key_id}>
                  {key.nickname}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

        </div>
        <Input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} className="flex-grow" />
        <TextAreaWithCopy className="flex-grow h-full" value={message} onChange={(e) => setMessage(e.target.value)} />
        <Button className="w-full" onClick={() => decryptMessage()}>
          Cook
        </Button>
        <TextAreaWithCopy value={outputMessage} disabled className="flex-grow h-full" />
      </TabsContent>
    );
  }

  return (
    <section>
      <Header />
      <div className="container">
        <Tabs defaultValue="encrypt" className="w-full w-max-[600px] flex flex-col items-center">
          <TabsList className="w-full">
            <TabsTrigger value="encrypt" className="flex-grow">
              ENCRYPT
            </TabsTrigger>
            <TabsTrigger value="decrypt" className="flex-grow">
              DECRYPT
            </TabsTrigger>
          </TabsList>

          {renderDecryptTab()}
          {renderEncryptTab()}
        </Tabs>
      </div>
    </section>
  );
}
