import { Category } from "./category";

export interface Card {
    categories: Category[],
    community_identifer: string,
    community_name: string,
    download_count: number,
    image_src?: string,
    is_deprecated: boolean,
    is_nsfw: boolean,
    last_updated: string,
    namespace: string,
    package_name: string,
    rating_score: number,
    team_name: string
}