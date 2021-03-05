use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(about = "Lists symlinks created by dot")]
    List,
    #[structopt(about = "Removes all symlinks")]
    Down {
        #[structopt(short, long)]
        dot_directory: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "dot")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,

    #[structopt(short, long)]
    pub bootstrap: bool,
}
