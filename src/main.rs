use clap::Parser;
use os_str_bytes::OsStrBytes;
use std::io::{stdout, Write};
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
        eprintln!("Error: {:#}", error);
        exit(1);
    }
}

fn cli(params: Params) -> anyhow::Result<()> {
    for parent in params.directories {
        walk(&parent)?;
    }

    Ok(())
}

fn walk(parent: &Path) -> anyhow::Result<usize> {
    let mut count: usize = 0;

    for entry in parent.read_dir()? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            count += walk(&entry.path())?
        }

        count += 1;
    }

    print!("{:6} ", count);
    stdout().write_all(&parent.to_raw_bytes())?;
    println!();

    Ok(count)
}
