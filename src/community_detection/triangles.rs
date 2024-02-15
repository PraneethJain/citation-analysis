use crate::common::*;

pub fn triangle_counts(graph: &Graph) -> Vec<usize> {
    let n = graph.adj_list.len();
    let mut counts = vec![0; n];
    let mut num_triangles = 0;
    for i in 0..n {
        for &j in &graph.adj_list[i] {
            for &k in &graph.adj_list[j] {
                if graph.adj_list[k].contains(&i) {
                    counts[i] += 1;
                    counts[j] += 1;
                    counts[k] += 1;
                    num_triangles += 1;
                }
            }
        }
    }
    println!("{}", num_triangles);
    return counts;
}
