use std::{
    fs,
    io::{stdout, BufWriter},
    path::Path,
};

use ferris_says::say;

fn main() {
    let file_path = Path::new("src/us/us.rs");
    let file = fs::read_to_string(file_path).unwrap();
    let stdout = stdout();
    let writer = BufWriter::new(stdout.lock());
    say(&file, 100, writer).unwrap();
}
