use std::collections::BTreeMap;
use std::{fs, io::Write};

fn main() {
    let src: String = fs::read_to_string("../src.txt").unwrap();
    let zj: String = fs::read_to_string("3500-table.txt").unwrap();

    // 过滤
    let mut lines: Vec<(&str, &str)> = src
        .lines()
        .filter_map(|v| {
            let s = v.split_once('\t').unwrap();
            if zj.contains(s.0) || s.1.contains('/') || s.1.contains('\'') {
                Some((s.1, s.0))
            } else {
                None
            }
        })
        .collect::<Vec<(&str, &str)>>();
    lines.sort_by_key(|k| k.0);

    let mut all: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for line in lines {
        let key = line.0;
        let ci = line.1;
        (*all.entry(key).or_insert(vec![])).push(ci);
    }

    // 合并
    let re: Vec<String> = all
        .iter()
        .map(|(k, v)| format!("{} {}", k, v.join(" ")))
        .collect();

    let mut buffer = fs::File::create("../3500-temp.txt").unwrap();
    buffer.write_all(re.join("\n").as_bytes()).unwrap();
}
