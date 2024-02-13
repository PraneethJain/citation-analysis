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

    // let ok = graph.clone();
    let _graphs = Graphs::new(graph, dates);
    // let g = graphs.till(&Date::from("0001-01-01"));
    // for i in 0..g.adj_list.len() {
    //     println!("{}", ok.index_to_id[&g.index_to_id[&i]]);
    // }
    // save_graphs(&graphs);
    // save_scc_counts("plots/scc_counts.txt", &graphs);
    // save_largest_scc_sizes("plots/largest_scc_sizes.txt", &graphs);
    // save_freeman_degree_centralization("plots/freeman_degree_centralization.txt", &graphs);
}
