use std::{fs, io::Write};

fn main() {
    let src: String = fs::read_to_string("/home/cy/.emacs.d/cyim/tool/cy.txt").unwrap(); 
    let mut lines: Vec<&str>  = src.lines().collect();
    let a = lines.sort_by_key(|k| {
        k.split_once(' ').unwrap().0});
    let mut buffer = fs::File::create("cyim.txt").unwrap();
    buffer.write_all(lines.join("\n").as_bytes()).unwrap();
}
