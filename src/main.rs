use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type Graph = BTreeMap<usize, Vec<usize>>;

fn parse(lines: &str) -> Graph {
    let mut adj_list: Graph =  BTreeMap::new();
    for line in lines.lines() {
        let (from, to): (usize, usize) = match line.split_once('\t') {
            Some((from, to)) => (from.parse().unwrap(), to.parse().unwrap()),
            None => panic!("No \t in line: {:?}", line),
        };
        adj_list.entry(from).or_default().push(to);
    }
    
    return adj_list;
}

fn main() {
    let path = Path::new("cit-HepPh.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut lines = String::new();

    if let Err(why) = file.read_to_string(&mut lines) {
        panic!("couldn't read {}: {}", display, why)
    }

    let graph = parse(&lines);
    println!("{:?}", graph);
}
