mod issue;
mod entities;
mod controller;

use clap::{Parser, Subcommand, ArgAction, command};

#[derive(Debug, Parser)]
pub struct Issue{
    #[clap(subcommand)]
    subcommand: IssueSubCommand
}

#[derive(Debug, Subcommand)]
pub enum IssueSubCommand{
    /// create an issue
    Create(IssueCreateArgs),

    /// List all issues
    List,

    /// delete an issue
    Delete(IssueDeleteArgs),
}


#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub struct IssueDeleteArgs{
    #[clap(short, long)]
    pub id: Option<i64>,

    #[clap(long)]
    pub all: bool,
}

#[derive(Debug, Parser)]
pub struct IssueCreateArgs {
    /// Issue Title
    #[clap(short, long)]
    pub title: String,

    /// Issue Title
    #[clap(short, long)]
    pub desc: Option<String>,
}

pub fn main(args: Issue) -> std::io::Result<()> {
    let _ = match args.subcommand {
        IssueSubCommand::Create(args) => controller::create(args),
        IssueSubCommand::List => controller::list(),
        IssueSubCommand::Delete(args) => controller::delete(args),
    };

    Ok(())
}

pub fn init() {
    issue::init();
}

