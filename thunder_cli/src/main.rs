use clap::{Parser, Subcommand};
use thunder_manager_common::utils::{
    installer::install_file,
    thunderstore::{
        community::{fetch_community_packages, list_communities},
        package::fetch_package,
    },
};

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(index = 1)]
        url: String,
    },
    Browse {
        #[arg(index = 1)]
        community_name: String,
    },
    Communities {},
    Show {
        #[arg(index = 1)]
        community_name: String,
        #[arg(index = 2)]
        mod_name: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to use (YAML or JSON)
    #[arg(short, long)]
    file_name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Browse { community_name }) => {
            let community_identifier = community_name.as_str();

            let packages = fetch_community_packages(community_identifier)
                .await
                .unwrap_or_else(|_| panic!("Failed to fetch packages"));

            for package in packages.packages {
                println!("{}", package.package_name);
            }
        }
        Some(Commands::Communities {}) => {
            let communities = list_communities(None)
                .await
                .unwrap_or_else(|err| panic!("Couldn't fetch communities: {err}"));

            for community in communities.results {
                println!("{}", community.name);
            }
        }
        Some(Commands::Show {
            community_name,
            mod_name,
        }) => {
            let package = fetch_package(mod_name.as_str(), community_name.as_str())
                .await
                .unwrap_or_else(|err| panic!("Couldn't fetch package: {err}"));

            println!(
                "{name}, {description}",
                name = package.name,
                description = package.latest.description
            );
        }
        None => {
            if args.file_name.is_some() {
                install_file(args.file_name.unwrap().as_str())
                    .await
                    .expect("Couldn't install mods");
            }
        }
        _ => {
            todo!()
        }
    }

    Ok(())
}
