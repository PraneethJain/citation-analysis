mod betweenness_centrality;
mod degree_centrality;
mod triangles;

pub use betweenness_centrality::*;
pub use degree_centrality::*;
pub use triangles::*;

fn measure(centralites: &[f32]) -> f32 {
    let max = centralites
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);
    centralites.iter().map(|&c| max - c).sum()
}

pub fn freeman_centralization(centralities: &[f32], max_centralities: &[f32]) -> f32 {
    measure(centralities) / measure(max_centralities)
}

pub fn create_star_graph_out(n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; n];
    for i in 1..n {
        adj_list[0].push(i);
    }
    adj_list
}

pub fn create_star_graph_in(n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; n];
    for i in 1..n {
        adj_list[i].push(0);
    }
    adj_list
}

pub fn create_star_graph_undirected(n: usize) -> Vec<Vec<usize>> {
    let mut adj_list = vec![vec![]; n];
    for i in 1..n {
        adj_list[i].push(0);
        adj_list[0].push(i);
    }
    adj_list
}
