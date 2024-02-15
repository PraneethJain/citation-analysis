use crate::common::*;
use std::collections::VecDeque;

pub fn betweenness_centralities(graph: &Graph) -> Vec<f32> {
    let n = graph.adj_list.len();
    let mut betweenness = vec![0.0; n];

    for s in 0..n {
        let mut S = Vec::new();
        let mut P = vec![Vec::new(); n];
        let mut sigma = vec![0; n];
        let mut d = vec![-1; n];
        sigma[s] = 1;
        d[s] = 0;
        let mut Q = VecDeque::new();
        Q.push_back(s);
        while let Some(v) = Q.pop_front() {
            S.push(v);
            for &w in &graph.adj_list[v] {
                if d[w] < 0 {
                    Q.push_back(w);
                    d[w] = d[v] + 1;
                }

                if d[w] == d[v] + 1 {
                    sigma[w] += sigma[v];
                    P[w].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = S.pop() {
            for &v in &P[w] {
                delta[v] += (1.0 + delta[w]) * sigma[v] as f32 / sigma[w] as f32;
            }

            if w != s {
                betweenness[w] += delta[w];
            }
        }
    }

    return betweenness;
}
