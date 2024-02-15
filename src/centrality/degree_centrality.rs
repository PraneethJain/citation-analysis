pub fn degree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<f32> {
    let mut degrees = vec![0.0; adj_list.len()];

    for (from, tos) in adj_list.iter().enumerate() {
        degrees[from] += tos.len() as f32;
        for &to in tos {
            degrees[to] += 1.0;
        }
    }

    degrees
}

pub fn indegree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<f32> {
    let mut indegrees = vec![0.0; adj_list.len()];

    for tos in adj_list.iter() {
        for &to in tos {
            indegrees[to] += 1.0;
        }
    }

    indegrees
}

pub fn outdegree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<f32> {
    adj_list.iter().map(|tos| tos.len() as f32).collect()
}
