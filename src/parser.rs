use crate::common::*;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_dates_map() -> BTreeMap<usize, Date> {
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
    let mut id_to_index: BTreeMap<usize, usize> = BTreeMap::new();
    let mut index_to_id: BTreeMap<usize, usize> = BTreeMap::new();

    let mut adj_list = vec![vec![]; MAX_NODES];
    let mut idx = 0;
    for line in lines.lines() {
        let (from, to): (usize, usize) = match line.split_once('\t') {
            Some((from, to)) => (from.parse().unwrap(), to.parse().unwrap()),
            None => panic!("No \t in line: {:?}", line),
        };

        for val in [from, to] {
            if !id_to_index.contains_key(&val) {
                id_to_index.insert(val, idx);
                index_to_id.insert(idx, val);
                idx += 1;
            }
        }

        adj_list[id_to_index[&from]].push(id_to_index[&to]);
    }

    Graph {
        adj_list,
        id_to_index,
        index_to_id,
    }
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

    lines
}
