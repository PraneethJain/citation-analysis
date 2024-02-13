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
    // save_graphs(&graphs);
    // save_scc_counts("plots/scc_counts.txt", &graphs);
    // save_largest_scc_sizes("plots/largest_scc_sizes.txt", &graphs);
    save_freeman_degree_centralization("plots/freeman_degree_centralization.txt", &graphs);
}
