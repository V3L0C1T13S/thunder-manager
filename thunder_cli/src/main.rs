use clap::Parser;
use thunder_manager_common::utils::installer::install_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to use (YAML or JSON)
    #[arg(short, long)]
    file_name: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    println!("ok so file is {}", args.file_name);
    install_file(args.file_name.as_str())
        .await
        .expect("Couldn't install mods");

    Ok(())
}
