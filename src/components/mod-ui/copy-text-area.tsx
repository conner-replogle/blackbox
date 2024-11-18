import { useState } from "react";
import { Textarea } from "@/components/ui/textarea";
import { ClipboardIcon } from "lucide-react";
import { Button } from "../ui/button";
export function TextAreaWithCopy(props:React.ComponentProps<"textarea">) {
  const [value, setValue] = useState("");

  const handleCopy = () => {
    navigator.clipboard.writeText(value).then(
      () => alert("Copied to clipboard!"), // Optional feedback
      (err) => alert("Failed to copy: " + err) // Optional error handling
    );
  };

  return (
    <div className={"relative " +props.className}>
      <Textarea
        
        placeholder="Type your message here."
        value={props.value || value}
        onChange={(e) => {
          setValue(e.target.value);
          if (props.onChange) {
            props.onChange(e);
          }
        }}
        className="w-full p-3"
        {...props}
      />
      
      <Button
      className="absolute top-1 right-1"
        onClick={handleCopy}
        variant={"outline"}
        size={"icon"}
        aria-label="Copy text"
      >
        <ClipboardIcon className="w-5 h-5" />
      </Button>
    </div>
  );
}
