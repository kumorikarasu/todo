mod task;
mod entities;
mod controller;

use clap::Subcommand;
use clap::Parser;

use crate::traits::System;

#[derive(Debug, Parser)]
pub struct Task{
    #[clap(subcommand)]
    subcommand: TaskSubCommand
}

#[derive(Debug, Subcommand)]
pub enum TaskSubCommand{
    /// create an issue
    Create(TaskCreateArgs),

    /// List all issues
    List,

    /// delete an issue
    Delete(TaskDeleteArgs),
}


#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub struct TaskDeleteArgs{
    #[clap(short, long)]
    pub id: Option<i64>,

    #[clap(long)]
    pub all: bool,
}

#[derive(Debug, Parser)]
pub struct TaskCreateArgs {
    /// Task Title
    #[clap(short, long)]
    pub title: String,

    /// Task Title
    #[clap(short, long)]
    pub desc: Option<String>,
}

pub async fn main(args: Task, backend: Box<dyn System>) -> std::io::Result<()> {
    let _ = match args.subcommand {
        TaskSubCommand::Create(args) => controller::create(args),
        TaskSubCommand::List => controller::list(backend).await,
        TaskSubCommand::Delete(args) => controller::delete(args),
    };

    Ok(())
}
