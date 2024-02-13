pub mod common;
pub mod graphviz;
pub mod parser;
pub mod plot;
pub mod scc;

pub use common::*;

fn dates() -> impl Iterator<Item = String> {
    (1992..=2002).flat_map(|year| (1..=12).map(move |month| format!("{:04}-{:02}-01", year, month)))
}

pub fn save_graphs(graphs: &Graphs) {
    for year in 1992..=2002 {
        for month in 1..=12 {
            let g = graphs.till(&Date::from(&format!("{:04}-{:02}-01", year, month)));
            graphviz::save(&format!("graphs/g{:04}-{:02}.gv", year, month), &g);
        }
    }
}

pub fn save_scc_counts(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let scc = scc::tarjan_scc(&g);
                (date_str, scc.len())
            })
            .collect::<Vec<_>>(),
    );
}

pub fn save_largest_scc_sizes(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let scc = scc::tarjan_scc(&g);
                (date_str, scc.iter().map(|x| x.len()).max().unwrap())
            })
            .collect::<Vec<_>>(),
    );
}
