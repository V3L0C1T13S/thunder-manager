export interface Community {
    identifier: string,
    name: string,
    discord_url?: string,
    wiki_url?: string,
    require_package_listing_approval?: boolean,
}

export interface ListResponse {
    next?: string,
    previous?: string,
    results?: Community[],
}