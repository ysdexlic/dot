# Dot

### TODO

- [ ] Learn Rust
- [ ] Write tests
- [ ] Error handling
  - [x] Add error when config file isn't set
  - [ ] Fix repeated use of printing the "No .dotrc found" error
  - [ ] Find out best error handling practices
  - [ ] Return pretty errors


- [x] Don't copy pesky system files like `.DS_Store` (done by only symlinking
  non dotted files)
  - [x] Seems to still be happening for nested dotfiles - i.e.
        `something/something/.DS_Store`
- [x] Copy folders/files recursively
  - [x] Don't make a symlink for the file, instead make the directories and only
    symlink the file


- [x] Add bootstrapping feature (run bootstrap executables)
- [ ] Add ability to have pre/post hooks
- [ ] Add ability to have host or tag specific files
- [ ] Add proper support for multiple dotfile dirs


- [x] Add regex file exclusions

- [x] Add down command to remove symlinks created by dot
  - [x] Down command should remove empty folders created by dot
- [x] Add list command to list symlinks created by dot


- [x] Save last state after upping
- [x] Consolidate with previous state to clean when upping
  - [x] Clean should remove empty folders created by dot
