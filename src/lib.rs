use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn get_keywords() -> Vec<String> {
    let file = File::open("./keywords").expect("error at get_keywords");
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap()).collect()
}
