use std::fs::remove_file;
use std::path::Path;

use crate::utils;

pub fn down_dotfiles(dot_dir: Option<String>) -> std::io::Result<()> {
    let links = utils::get_links();

    for link in links {
        let (_, link_out) = link;
        if Path::new(&link_out).exists() {
            remove_file(&link_out)?;
        }
    }

    Ok(())
}
