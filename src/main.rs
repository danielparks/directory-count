use clap::Parser;
use os_str_bytes::OsStrBytes;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use walkdir::WalkDir;

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
    let mut stdout = std::io::stdout();
    let mut depth_count: Vec<usize> = Vec::new();

    for parent in params.directories {
        for entry in WalkDir::new(parent).contents_first(true) {
            let entry = entry?; // FIXME print error and continue?
            let depth = entry.depth();

            print!("{:6} ", if entry.file_type().is_dir() { "d" } else if entry.file_type().is_symlink() { "s" } else { "f" });
            stdout.write_all(&entry.path().to_raw_bytes())?;
            println!("");

            if depth >= depth_count.len() {
                // Depth increased.
                depth_count.resize(depth+1, 0);
            } else if depth+1 < depth_count.len() {
                // Depth decreased; we finished looking through sub directory.
                //assert!(entry.file_type().is_dir(), "Left directory but next entry was not directory itself");
                if !entry.file_type().is_dir() {
                    eprintln!("Left directory but next entry was not directory itself:");
                }

                let sum: usize = depth_count.drain(depth+1..).sum();

                print!("{:6} ", sum);
                stdout.write_all(&entry.path().to_raw_bytes())?;
                println!("");

                depth_count[depth] += sum;
            } else if entry.file_type().is_dir() {
                // Directory with no contents.
                print!("{:6} ", 0);
                stdout.write_all(&entry.path().to_raw_bytes())?;
                println!("");
            }

            depth_count[depth] += 1;
        }
    }
    Ok(())
}
