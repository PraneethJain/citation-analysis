mod common;
mod graphviz;
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
    for year in 1992..=2003 {
        for month in 1..=12 {
            let g = graphs.till(&Date::from(&format!("{:04}-{:02}-01", year, month)));
            graphviz::save(&format!("graphs/g{:04}-{:02}.gv", year, month), &g);
        }
    }
}
