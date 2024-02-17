use rayon::prelude::*;
use std::collections::{BTreeMap, VecDeque};

fn edge_betweenness_centralities(adj_list: &Vec<Vec<usize>>) -> BTreeMap<(usize, usize), f32> {
    let n = adj_list.len();

    // single source shortest paths
    (0..n)
        .into_par_iter()
        .map(|s| {
            let mut betweenness = BTreeMap::new();
            let mut stack = Vec::new();
            let mut pred = vec![Vec::new(); n];
            let mut sigma = vec![0; n];
            let mut dist = vec![-1; n];
            sigma[s] = 1;
            dist[s] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(s);
            while let Some(v) = queue.pop_front() {
                stack.push(v);
                for &w in &adj_list[v] {
                    // path discovery
                    if dist[w] < 0 {
                        queue.push_back(w);
                        dist[w] = dist[v] + 1;
                    }

                    // path counting
                    if dist[w] == dist[v] + 1 {
                        sigma[w] += sigma[v];
                        pred[w].push(v);
                    }
                }
            }

            // accumulation
            let mut delta = vec![0.0; n];
            while let Some(w) = stack.pop() {
                for &v in &pred[w] {
                    let c = (1.0 + delta[w]) * sigma[v] as f32 / sigma[w] as f32;
                    *betweenness.entry((v, w)).or_default() += c;
                    delta[v] += c;
                }
            }
            betweenness
        })
        .reduce(BTreeMap::new, |mut x, y| {
            for (k, v) in y {
                *x.entry(k).or_insert(0.0) += v;
            }
            x
        })
}

pub fn girwan_newman(adj_list: &Vec<Vec<usize>>, max_cluster_size: usize) -> Vec<Vec<usize>> {
    let mut working_adj_list = adj_list.clone();
    loop {
        let prev_sccs = super::scc::tarjan_scc(&working_adj_list);
        let edge_centralities = edge_betweenness_centralities(&working_adj_list);
        if let Some((&(v, w), _)) = edge_centralities
            .iter()
            .max_by(|&(_, &c1), &(_, c2)| c1.partial_cmp(c2).unwrap())
        {
            working_adj_list[v].retain(|&x| x != w);
            working_adj_list[w].retain(|&x| x != v);

            let sccs = super::scc::tarjan_scc(&working_adj_list);
            println!(
                "{:?}",
                prev_sccs.iter().map(|x| x.len()).collect::<Vec<_>>()
            );
            println!("{}", prev_sccs.len());
            match sccs.len().cmp(&max_cluster_size) {
                std::cmp::Ordering::Greater => break prev_sccs,
                std::cmp::Ordering::Equal => break sccs,
                std::cmp::Ordering::Less => (),
            }
        } else {
            break prev_sccs;
        }
    }
}
