mod betweenness_centrality;
mod degree_centrality;
mod triangles;

pub use betweenness_centrality::*;
pub use degree_centrality::*;
pub use triangles::*;

use std::ops::Sub;
fn measure<T>(centralites: &[T]) -> T
where
    T: std::iter::Sum + Sub<Output = T> + Ord + Copy,
{
    let max = *centralites.iter().max().unwrap();
    centralites.iter().map(|&c| max - c).sum()
}

pub fn freeman_centralization(centralities: &[usize], max_centralities: &[usize]) -> f32 {
    measure(centralities) as f32 / measure(max_centralities) as f32
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
