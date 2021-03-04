use colour::{blue, cyan};

use crate::utils;

pub fn list_dotfiles() -> std::io::Result<()> {
    let links = utils::get_links()?;
    for link in links {
        blue!("{}", link.0);
        print!(" -> ");
        cyan!("{}\n", link.1);
    }
    Ok(())
}
