import { CopyIcon, Trash, UserCircle } from "lucide-react"
import { Card, CardContent } from "../ui/card"
import { Button } from "../ui/button"
import { Badge } from "../ui/badge"
import { Key } from "@/lib/api/types";

interface IdentityProps {
    Key: Key;

    onDelete: () =>void;
}

export function Identity({Key,onDelete}:IdentityProps) {
    return(
    <Card key={Key.nickname} className="group">
        <CardContent className="flex items-center justify-between p-4">
            <div className="flex items-center gap-3">
            <UserCircle className="text-muted-foreground" />
            <span className="font-medium ">{Key.nickname} </span>
            </div>
            <span className="flex justify-center items-center gap-3 h-8">
                {Key.is_me &&<Badge variant="outline">Identity</Badge>} 
                <Button
                variant="ghost"
                size="icon"
                className="hidden group-hover:flex transition-opacity"
                onClick={() => {navigator.clipboard.writeText(Key.public_key)}}
                >
                <CopyIcon className="h-4 w-4 " />
                </Button>

        { !Key.is_me && <Button
            variant="ghost"
            size="icon"
            className="opacity-0 group-hover:opacity-100 transition-opacity"
            onClick={onDelete}
            >
            <Trash className="h-4 w-4 text-destructive" />
            </Button>}
            </span>
        </CardContent>
    </Card>

    )
}