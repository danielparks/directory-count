# Recursively count the contents of directories recursively

Think `du`, but it counts directory entries, e.g. files, directories, symlinks,
instead of the size of files.

### Example

```
❯ directory_count /etc/ssh
     1 /etc/ssh/ssh_config.d
     2 /etc/ssh/sshd_config.d
     7 /etc/ssh
❯ tree -F /etc/ssh
/etc/ssh/
├── moduli
├── ssh_config
├── ssh_config.d/
├── sshd_config
└── sshd_config.d/
    └── 100-macos.conf

3 directories, 4 files
```

## License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.
