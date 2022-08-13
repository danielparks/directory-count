use anyhow::Result;
use clap::Parser;
use os_str_bytes::OsStrBytes;
use std::fmt::Display;
use std::io::{stderr, stdout, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
struct Params {
    /// Directory to count
    #[clap()]
    directories: Vec<PathBuf>,
}

fn main() {
    if let Err(error) = cli(Params::parse()) {
        print_error(None, &error).unwrap();
        exit(1);
    }
}

fn cli(params: Params) -> Result<()> {
    for parent in params.directories {
        if let Err(error) = walk(&parent) {
            print_error(Some(&parent), &error)?;
        }
    }

    Ok(())
}

fn walk(parent: &Path) -> Result<usize> {
    let mut count: usize = 1; // Count this directory.

    for entry in parent.read_dir()? {
        count += match entry {
            Ok(entry) => {
                if entry.file_type()?.is_dir() {
                    match walk(&entry.path()) {
                        Ok(dir_count) => dir_count,
                        Err(error) => {
                            print_error(Some(&entry.path()), &error)?;
                            1
                        }
                    }
                } else {
                    1
                }
            }
            Err(error) => {
                // FIXME should this at least show the parent path?
                print_error(None, &error)?;
                1
            }
        }
    }

    print!("{:6} ", count);
    stdout().write_all(&parent.to_raw_bytes())?;
    println!();

    Ok(count)
}

fn print_error<E>(path: Option<&Path>, error: &E) -> Result<()>
where
    E: Display,
{
    eprint!("directory-count: ");
    if let Some(path) = path {
        stderr().write_all(&path.to_raw_bytes())?;
        eprintln!(": {}", error);
    } else {
        eprintln!("{}", error);
    }

    Ok(())
}
