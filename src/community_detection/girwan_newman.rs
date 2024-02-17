use std::collections::{BTreeMap, VecDeque};

fn edge_betweenness_centralities(adj_list: &Vec<Vec<usize>>) -> BTreeMap<(usize, usize), f32> {
    let n = adj_list.len();
    let mut betweenness = BTreeMap::new();

    for s in 0..n {
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
                if dist[w] < 0 {
                    queue.push_back(w);
                    dist[w] = dist[v] + 1;
                }

                if dist[w] == dist[v] + 1 {
                    sigma[w] += sigma[v];
                    pred[w].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w] {
                let c = (1.0 + delta[w]) * sigma[v] as f32 / sigma[w] as f32;
                *betweenness.entry((v, w)).or_default() += c;
                delta[v] += c;
            }
        }
    }

    return betweenness;
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
            if sccs.len() != prev_sccs.len() {
                println!(
                    "{} {:?}",
                    sccs.len(),
                    sccs.iter().map(|x| x.len()).collect::<Vec<_>>()
                );
            }
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
