import { Card } from "./card";

export interface PackageList {
    bg_image_src?: string,
    categories: never[],
    community_name: string,
    has_more_pages: boolean,
    packages: Card[],
}