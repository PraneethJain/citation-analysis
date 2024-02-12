use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::common::*;

pub fn parse() -> Graph {
    let id_to_dates: BTreeMap<u64, Date> = read_file("cit-HepPh-dates.txt")
        .lines()
        .map(|line| match line.split_once('\t') {
            Some((id, date)) => (id.parse().unwrap(), Date::from(date)),
            None => panic!("No \t in line: {:?}", line),
        })
        .collect();

    let lines = read_file("cit-HepPh.txt");
    let mut adj_list: Graph = BTreeMap::new();
    for line in lines.lines() {
        let (from, to): (Node, Node) = match line.split_once('\t') {
            Some((from, to)) => {
                let from_id: u64 = from.parse().unwrap();
                let from_date = id_to_dates
                    .get(&from_id)
                    .unwrap_or(&Date::new())
                    .clone();

                let to_id: u64 = to.parse().unwrap();
                let to_date = id_to_dates
                    .get(&to_id)
                    .unwrap_or(&Date::new())
                    .clone();
                (
                    Node {
                        id: from_id,
                        date: from_date,
                    },
                    Node {
                        id: to_id,
                        date: to_date,
                    },
                )
            }
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
