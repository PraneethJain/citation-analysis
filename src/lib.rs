pub mod centrality;
pub mod common;
pub mod community_detection;
pub mod graphviz;
pub mod parser;
pub mod plot;

pub use common::*;

fn dates() -> impl Iterator<Item = String> {
    (1992..=2002).flat_map(|year| (1..=12).map(move |month| format!("{:04}-{:02}-01", year, month)))
}

pub fn save_graphs(graphs: &Graphs) {
    dates().for_each(|date_str| {
        let filename = String::from("graphs/g") + &date_str[..7] + ".gv";
        graphviz::save(&filename, &graphs.till(&Date::from(&date_str)));
    });
}

pub fn save_scc_counts(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let scc = community_detection::scc::tarjan_scc(&g);
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
                let scc = community_detection::scc::tarjan_scc(&g);
                (date_str, scc.iter().map(|x| x.len()).max().unwrap())
            })
            .collect::<Vec<_>>(),
    );
}

pub fn save_freeman_degree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let degree_centralities = centrality::degree_centralities(&g.adj_list);
                let max_degree_centralities = centrality::degree_centralities(
                    &centrality::create_star_graph_undirected(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &degree_centralities,
                    &max_degree_centralities,
                );
                (date_str, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_indegree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let indegree_centralities = centrality::indegree_centralities(&g.adj_list);
                let max_indegree_centralities = centrality::indegree_centralities(
                    &centrality::create_star_graph_undirected(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &indegree_centralities,
                    &max_indegree_centralities,
                );
                (date_str, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_outdegree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date_str| {
                let g = graphs.till(&Date::from(&date_str));
                let outdegree_centralities = centrality::outdegree_centralities(&g.adj_list);
                let max_outdegree_centralities = centrality::outdegree_centralities(
                    &centrality::create_star_graph_out(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &outdegree_centralities,
                    &max_outdegree_centralities,
                );
                (date_str, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}
