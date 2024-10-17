import { browser } from "$app/environment";
import { writable, type Writable } from "svelte/store";


export function restorable<T>(id: string, initial: T): Writable<T> {
    let restored = sessionStorage.getItem(id);
    const out = writable((browser && restored && JSON.parse(restored)) || initial);
    out.subscribe((val) => {
        if (browser) return (sessionStorage[id] = JSON.stringify(val));
    })
    return out;
}   