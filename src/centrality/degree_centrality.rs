pub fn degree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut degrees = vec![0; adj_list.len()];

    for (from, tos) in adj_list.iter().enumerate() {
        degrees[from] += tos.len();
        for &to in tos {
            degrees[to] += 1;
        }
    }

    degrees
}

pub fn indegree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut indegrees = vec![0; adj_list.len()];

    for tos in adj_list.iter() {
        for &to in tos {
            indegrees[to] += 1;
        }
    }

    indegrees
}

pub fn outdegree_centralities(adj_list: &Vec<Vec<usize>>) -> Vec<usize> {
    adj_list.iter().map(|tos| tos.len()).collect()
}
