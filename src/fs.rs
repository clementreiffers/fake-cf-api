use std::fs::File;
use std::io::Read;

pub fn read_json(path: &str) -> String {
    let mut contents = String::new();
    File::open(path)
        .expect("failed to open file")
        .read_to_string(&mut contents)
        .expect("failed to get content from file");
    contents
}
