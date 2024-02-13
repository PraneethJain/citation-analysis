use crate::common::*;

pub fn degree_centralities(graph: &Graph) -> Vec<usize> {
    let mut degrees = vec![0; graph.adj_list.len()];

    for (from, tos) in graph.adj_list.iter().enumerate() {
        degrees[from] += tos.len();
        for &to in tos {
            degrees[to] += 1;
        }
    }

    degrees
}
