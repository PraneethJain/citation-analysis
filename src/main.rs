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
    save_largest_scc_sizes("plots/largest_scc_sizes.txt", &graphs);
}
