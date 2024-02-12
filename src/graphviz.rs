use crate::common::*;
use std::fs;

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
