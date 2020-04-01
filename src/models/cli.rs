use structopt::*;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(Debug, StructOpt)]
pub enum Command {
    AddUser {
        #[structopt(short, long)]
        username: String,
        #[structopt(short, long)]
        password: String
    },
    DeleteUser {
        #[structopt(short, long)]
        username: String
    }
}