mod common;
mod parser;
mod scc;

use common::*;

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
    let some_graph = graphs.till(&Date::from("2001-03-31"));
    println!("{}", some_graph.adj_list.len());
}
