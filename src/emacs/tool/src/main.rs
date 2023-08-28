use std::{fs, io::Write};

fn main() {
    let src: String = fs::read_to_string("/home/cy/.emacs.d/cyim/tool/i.sql").unwrap(); 
    let mut end: String = String::new();
    let mut one: &str;
    let mut temp: &str = "";
    let mut two: &str;
    for line in src.lines() {
        let (one, two) = line.split_once(' ').unwrap();
        if temp == one {
            end.push(' ');
            end.push_str(two);
        } else {
            temp = one;
            end.push('\n');
            //end.push_str(one);
            //end.push_str(" ");
            end.push_str(line);
        }
    }
    let mut buffer = fs::File::create("cy.txt").unwrap();
    buffer.write_all(end.as_bytes()).unwrap();
    println!("ok");
/*
    let mut lines: Vec<&str>  = src.lines().collect();
    let a = lines.sort_by_key(|k| {
        k.split_once(' ').unwrap().0});
    let mut buffer = fs::File::create("cy.txt").unwrap();
    buffer.write_all(lines.join("\n").as_bytes()).unwrap();
    */
}
