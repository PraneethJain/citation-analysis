use citation_analysis::*;

fn main() {
    let graph = parser::parse();
    let dates_map = parser::get_dates_map();
    let dates: Vec<_> = (0..MAX_NODES)
        .map(|i| {
            dates_map
                .get(&graph.index_to_id[&i])
                .unwrap_or(&Date::old())
                .clone()
        })
        .collect();

    let graphs = Graphs::new(graph, dates);
    let g = graphs.till(&Date::from("1994-01-01"));
    let largest_scc_graph = community_detection::largest_scc_graph(&g);
    let communities = community_detection::girwan_newman(&largest_scc_graph.adj_list, 7);
    graphviz::save_with_colors(
        "community_graphs/girwan_newman/g1994.gv",
        &largest_scc_graph.adj_list,
        &communities,
    );

    // save_graphs(&graphs);
    // save_scc_counts("plots/scc_counts.txt", &graphs);
    // save_largest_scc_sizes("plots/largest_scc_sizes.txt", &graphs);

    // save_freeman_degree_centralization("plots/freeman_degree_centralization.txt", &graphs);
    // save_freeman_indegree_centralization("plots/freeman_indegree_centralization.txt", &graphs);
    // save_freeman_outdegree_centralization("plots/freeman_outdegree_centralization.txt", &graphs);
    // save_freeman_betweenness_centralization(
    //     "plots/freeman_betweenness_centralization.txt",
    //     &graphs,
    // );
}
