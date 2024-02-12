use crate::common::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_dates_map() -> BTreeMap<u64, Date> {
    read_file("cit-HepPh-dates.txt")
        .lines()
        .map(|line| match line.split_once('\t') {
            Some((id, date)) => (id.parse().unwrap(), Date::from(date)),
            None => panic!("No \t in line: {:?}", line),
        })
        .collect()
}

pub fn parse() -> Graph {
    let lines = read_file("cit-HepPh.txt");
    let mut adj_list: Graph = BTreeMap::new();
    for line in lines.lines() {
        let (from, to): (u64, u64) = match line.split_once('\t') {
            Some((from, to)) => (from.parse().unwrap(), to.parse().unwrap()),
            None => panic!("No \t in line: {:?}", line),
        };
        adj_list.entry(from).or_default().push(to);
    }

    return adj_list;
}

fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();

    if let Err(why) = file.read_to_string(&mut lines) {
        panic!("couldn't read {}: {}", display, why)
    }

    return lines;
}
