
mod task;
mod config;
mod clickup;
mod error;
mod traits;

use tokio;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct AppArgs {

    #[clap(subcommand)]
    pub subcmd: AppSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum AppSubCommand {
    #[clap(about = "Manage tasks")]
    Task(task::Task),
}

#[derive(Debug, Parser)]
pub struct RunArgs {
    /// The port to listen on
    #[clap(short, long)]
    pub port: Option<u16>,
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let args = AppArgs::parse();
    let backend: Box<dyn traits::System>;

    match config::init() {
        Ok(_) => (),
        Err(e) => panic!("Error: {}", e),
    }

    if config::get_system() == "clickup" {
        backend = Box::new(clickup::new());
    }else{
        panic!("System not supported");
    }

    let _ = match args.subcmd {
        AppSubCommand::Task(args) => task::main(args, backend).await,
    };

    Ok(())
}
