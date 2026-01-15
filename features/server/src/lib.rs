pub mod args {
    pub mod add_args;
    pub mod default_args;
    pub mod get_args;
    pub mod list_args;
    pub mod remove_args;
}

pub mod cmd {
    pub mod add;
    pub mod default;
    pub mod get;
    pub mod list;
    pub mod remove;
}

use args::{
    add_args::ServerAddArgs, default_args::ServerDefaultArgs, get_args::ServerGetArgs,
    list_args::ServerListArgs, remove_args::ServerRemoveArgs,
};

pub enum ServerCommand {
    List(ServerListArgs),
    Add(ServerAddArgs),
    Default(ServerDefaultArgs),
    Remove(ServerRemoveArgs),
    Get(ServerGetArgs),
}

pub fn execute(cmd: ServerCommand) {
    match cmd {
        ServerCommand::Add(args) => cmd::add::run(args),
        ServerCommand::List(args) => cmd::list::run(args),
        ServerCommand::Default(args) => cmd::default::run(args),
        ServerCommand::Remove(args) => cmd::remove::run(args),
        ServerCommand::Get(args) => cmd::get::run(args),
    }
}
