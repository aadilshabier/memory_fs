use lib::create_root;
mod lib;

fn main() {
    let mut root = create_root();

    let mut filename = String::from("");
    let a = root.ls(&filename);

    match a {
        Ok(files) => {
            for file in files {
                print!("{}", file);
            }
        },
        Err(()) => eprintln!("ERROR: Could not ls(\"{}\")", filename)
    }
}
