use crate::common::*;
use std::fs;

const COLORS: [&str; 64] = [
    "#000000", "#00FF00", "#0000FF", "#FF0000", "#01FFFE", "#FFA6FE", "#FFDB66", "#006401",
    "#010067", "#95003A", "#007DB5", "#FF00F6", "#FFEEE8", "#774D00", "#90FB92", "#0076FF",
    "#D5FF00", "#FF937E", "#6A826C", "#FF029D", "#FE8900", "#7A4782", "#7E2DD2", "#85A900",
    "#FF0056", "#A42400", "#00AE7E", "#683D3B", "#BDC6FF", "#263400", "#BDD393", "#00B917",
    "#9E008E", "#001544", "#C28C9F", "#FF74A3", "#01D0FF", "#004754", "#E56FFE", "#788231",
    "#0E4CA1", "#91D0CB", "#BE9970", "#968AE8", "#BB8800", "#43002C", "#DEFF74", "#00FFC6",
    "#FFE502", "#620E00", "#008F9C", "#98FF52", "#7544B1", "#B500FF", "#00FF78", "#FF6E41",
    "#005F39", "#6B6882", "#5FAD4E", "#A75740", "#A5FFD2", "#FFB167", "#009BFF", "#E85EBE",
];

pub fn save(filename: &str, graph: &Graph) {
    let mut content = String::from("digraph g {\n\tnode[label=\"\"]\n");
    for (from, tos) in graph.adj_list.iter().enumerate() {
        for to in tos {
            content += &format!("\t{} -> {};\n", from, to);
        }
    }
    content += "}";

    if let Err(e) = fs::write(filename, content) {
        eprintln!("Error writing to file: {}", e);
    }
}

pub fn save_with_colors(filename: &str, adj_list: &Vec<Vec<usize>>, partitions: &Vec<Vec<usize>>) {
    let n = adj_list.len();
    let mut index_to_partition = vec![0; n];
    let mut better_partitions = vec![Vec::new()];
    for partition in partitions {
        if partition.len() <= 2 {
            better_partitions[0].extend_from_slice(partition);
        } else {
            better_partitions.push(partition.to_vec());
        }
    }
    better_partitions.sort();

    for (partition, nodes) in better_partitions.into_iter().enumerate() {
        for index in nodes {
            index_to_partition[index] = partition;
        }
    }
    let mut content = String::from("digraph g {\n\tnode[label=\"\"]\n");
    for i in 0..adj_list.len() {
        content += &format!(
            "\t {} [color=\"{}\", fillcolor=\"{}\", style=filled]\n",
            i, COLORS[index_to_partition[i]], COLORS[index_to_partition[i]]
        );
    }
    for (from, tos) in adj_list.iter().enumerate() {
        for to in tos {
            content += &format!("\t{} -> {};\n", from, to);
        }
    }
    content += "}";

    if let Err(e) = fs::write(filename, content) {
        eprintln!("Error writing to file: {}", e);
    }
}
