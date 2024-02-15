use std::collections::VecDeque;

pub fn betweenness_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<f32> {
    let n = adj_list.len();
    let mut betweenness = vec![0.0; n];

    for s in 0..n {
        let mut stack = Vec::new();
        let mut p = vec![Vec::new(); n];
        let mut sigma = vec![0; n];
        let mut d = vec![-1; n];
        sigma[s] = 1;
        d[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &w in &adj_list[v] {
                if d[w] < 0 {
                    queue.push_back(w);
                    d[w] = d[v] + 1;
                }

                if d[w] == d[v] + 1 {
                    sigma[w] += sigma[v];
                    p[w].push(v);
                }
            }
        }

        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &p[w] {
                delta[v] += (1.0 + delta[w]) * sigma[v] as f32 / sigma[w] as f32;
            }

            if w != s {
                betweenness[w] += delta[w];
            }
        }
    }

    return betweenness;
}
