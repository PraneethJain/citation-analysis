pub mod common;
pub mod graphviz;
pub mod parser;
pub mod plot;
pub mod scc;

pub use common::*;

pub fn save_graphs(graphs: &Graphs) {
    for year in 1992..=2002 {
        for month in 1..=12 {
            let g = graphs.till(&Date::from(&format!("{:04}-{:02}-01", year, month)));
            graphviz::save(&format!("graphs/g{:04}-{:02}.gv", year, month), &g);
        }
    }
}

pub fn save_scc_counts(filename: &str, graphs: &Graphs) {
    let mut x_vals = vec![];
    let mut y_vals = vec![];
    for year in 1992..=2002 {
        for month in 1..=12 {
            let date_str = format!("{:04}-{:02}-01", year, month);
            let g = graphs.till(&Date::from(&date_str));
            let scc = scc::tarjan_scc(&g);
            x_vals.push(date_str);
            y_vals.push(scc.len());
        }
    }
    plot::line_plot(filename, &x_vals, &y_vals);
}

pub fn save_largest_scc_sizes(filename: &str, graphs: &Graphs) {
    let mut x_vals = vec![];
    let mut y_vals = vec![];
    for year in 1992..=2002 {
        for month in 1..=12 {
            let date_str = format!("{:04}-{:02}-01", year, month);
            let g = graphs.till(&Date::from(&date_str));
            let scc = scc::tarjan_scc(&g);
            x_vals.push(date_str);
            y_vals.push(scc.iter().map(|x| x.len()).max().unwrap());
        }
    }
    plot::line_plot(filename, &x_vals, &y_vals);
}
