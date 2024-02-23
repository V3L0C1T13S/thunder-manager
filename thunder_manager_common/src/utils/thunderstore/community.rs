use thunderstore_api::{
    apis::{
        configuration::Configuration,
        v2::community::{self, ListError},
        Error,
    },
    models::v2::community::{ListResponse, PackageList},
};

pub type ListCommunitiesResponse = ListResponse;
pub type ListCommunitiesError = Error<ListError>;

pub type FetchCommunityPackagesResponse = PackageList;

pub async fn list_communities(cursor: Option<&str>) -> Result<ListResponse, ListCommunitiesError> {
    let config = Configuration::new();
    let communities = community::list(&config, cursor).await;

    if let Ok(communities) = communities {
        Ok(communities)
    } else {
        communities
    }
}

pub async fn fetch_community_packages(
    community_identifier: &str,
) -> Result<FetchCommunityPackagesResponse, ()> {
    let config: Configuration = Configuration::new();
    let packages = community::list_packages(&config, community_identifier).await;

    if let Ok(packages) = packages {
        Ok(packages)
    } else {
        // TODO: errors
        Err(())
    }
}
