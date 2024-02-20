import { invoke } from "@tauri-apps/api/tauri";
import { ListResponse } from "./community";
import { PackageList } from "./package";

export enum Commands {
    InstallFile = "install_file"
}

let communities: Record<string, ListResponse> = {};

export async function ListCommunities(cursor?: string) {
    if (communities[cursor ?? "0"]) return communities[cursor ?? "0"];

    console.log("fetching communities (fresh)");
    communities[cursor ?? "0"] = await invoke<ListResponse>("list_communities", {
        cursor
    });

    return communities[cursor ?? "0"];
}

export async function GetCommunityPackages(identifier: string) {
    console.log("fetching fresh packages...");

    const packages = await invoke<PackageList>("fetch_community", {
        identifier
    });

    return packages;
}