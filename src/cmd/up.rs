use std::fs::{read_link, remove_file, rename, symlink_metadata};
use std::os::unix::fs::symlink;
use std::path::Path;

use crate::utils;

pub fn link_dotfiles() -> std::io::Result<()> {
    let links = utils::get_links();

    for link in links {
        let (link_in, link_out) = link;
        if Path::new(&link_out).exists() {
            let metadata = symlink_metadata(&link_out)?;
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                if read_link(&link_out).unwrap().to_str().unwrap() == link_in {
                    continue;
                }
                remove_file(&link_out)?;
            }
            if file_type.is_file() {
                let new_out_path = &format!("{}.old", link_out);
                rename(&link_out, new_out_path)?;
            }
        }
        symlink(&link_in, &link_out)?;
    }

    println!("All Done!");
    Ok(())
}
