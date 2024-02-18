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
    save_louvain_graphs(&graphs);
    //
    // save_graphs(&graphs);
    // save_scc_counts("plots/scc_counts.txt", &graphs);
    // save_largest_scc_sizes("plots/largest_scc_sizes.txt", &graphs);
    // let r2_score = save_linear_regression_largest_wcc_sizes("plots/wcc_size_linreg.txt", &graphs);
    //
    // save_max_degree_centralization("plots/max_degree_centralization.txt", &graphs);
    // save_max_indegree_centralization("plots/max_indegree_centralization.txt", &graphs);
    // save_max_outdegree_centralization("plots/max_outdegree_centralization.txt", &graphs);
    // save_max_betweenness_centralization("plots/max_betweenness_centralization.txt", &graphs);
    // save_freeman_degree_centralization("plots/freeman_degree_centralization.txt", &graphs);
    // save_freeman_indegree_centralization("plots/freeman_indegree_centralization.txt", &graphs);
    // save_freeman_outdegree_centralization("plots/freeman_outdegree_centralization.txt", &graphs);
    // save_freeman_betweenness_centralization(
    //     "plots/freeman_betweenness_centralization.txt",
    //     &graphs,
    // );
}
