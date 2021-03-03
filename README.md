# Dot

### TODO

- [ ] Learn Rust
- [ ] Error handling


- [x] Don't copy pesky system files like `.DS_Store` (done by only symlinking
  non dotted files)
- [ ] Copy folders/files recursively
  - [ ] Don't make a symlink for the file, instead make the directories and only
    symlink the file.


- [ ] Add bootstrapping feature (run bootstrap executables)
- [ ] Add ability to have pre/post hooks
- [ ] Add ability to have host or tag specific files


- [x] Add regex file exclusions

- [ ] Add down command to remove symlinks created by dot
- [x] Add list command to list symlinks created by dot


- [x] Save last state after upping
- [ ] Consolidate with previous state to clean when upping
