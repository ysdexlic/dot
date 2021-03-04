use structopt::StructOpt;

mod cmd;
mod input;
mod state;
mod utils;

fn main() -> std::io::Result<()> {
    state::setup()?;

    let args = input::Cli::from_args();

    let res = match args.cmd {
        Some(input::Command::List) => cmd::list::list_dotfiles(),
        Some(input::Command::Initialize) => cmd::init::init_dotfiles(),
        Some(input::Command::Down { dot_directory }) => cmd::down::down_dotfiles(dot_directory),
        None => cmd::up::link_dotfiles(args.bootstrap),
    };

    state::clean()?;
    state::set_state()?;

    res.expect("something went wrong");

    Ok(())
}
