import { Card } from "./card";

export interface PackageList {
    bg_image_src?: string,
    categories: never[],
    community_name: string,
    has_more_pages: boolean,
    packages: Card[],
}

export interface PackageVersionExperimental {
    namespace?: string,
    name: string,
    description: string,
}

export interface ThunderstorePackage {
    namespace?: string,
    name: string,
    full_name: string,
    owner?: string,
    package_url?: string,
    date_created?: string,
    date_updated?: string,
    rating_score?: string,
    is_pinned?: boolean,
    is_deprecated?: boolean,
    total_downloads?: string,
    latest: PackageVersionExperimental,
    community_listings?: never[]
}

export interface ThunderstorePackageV1 {
    name?: string,
    full_name?: string,
    owner?: string,
    package_url?: string,
    donation_link?: string,
    date_created?: string,
    date_updated?: string,
    uuid4?: string,
    is_pinned?: string,
    is_deprecated?: string,
    has_nsfw_content?: boolean,
    categories?: string,
    versions?: string,
}

export type PackageListV1 = ThunderstorePackageV1[];
