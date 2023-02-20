mod lib;

use lib::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut root = create_root();

    let files = root.ls("")?;
    println!("New filesystem, should be empty:");
    for file in files {
        println!("{}", file);
    }

    root.touch("file1.txt")?;
    root.mkdir("dir")?;

    root.write(
        "dir/file2",
        &[72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100],
    )?;

    println!("Should have 2 entries:");
    let files = root.ls("")?;
    for file in files {
        println!("{}", file);
    }

    Ok(())
}
