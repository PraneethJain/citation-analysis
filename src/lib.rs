pub mod centrality;
pub mod common;
pub mod community_detection;
pub mod graphviz;
pub mod parser;
pub mod plot;

pub use common::*;

fn dates() -> impl Iterator<Item = Date> {
    (1992..=2002).flat_map(|year| (1..=12).map(move |month| Date::new(year, month, 1)))
}

pub fn save_graphs(graphs: &Graphs) {
    dates().for_each(|date| {
        let filename =
            String::from("graphs/g") + &format!("{:04}-{:02}", date.year, date.month) + ".gv";
        graphviz::save(&filename, &graphs.till(&date));
    });
}

pub fn save_scc_counts(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let scc = community_detection::tarjan_scc(&g.adj_list);
                (date, scc.len())
            })
            .collect::<Vec<_>>(),
    );
}

pub fn save_largest_scc_sizes(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let scc = community_detection::tarjan_scc(&g.adj_list);
                (date, scc.iter().map(|x| x.len()).max().unwrap())
            })
            .collect::<Vec<_>>(),
    );
}

pub fn save_freeman_degree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let degree_centralities = centrality::degree_centralities(&g.adj_list);
                let max_degree_centralities = centrality::degree_centralities(
                    &centrality::create_star_graph_undirected(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &degree_centralities,
                    &max_degree_centralities,
                );
                (date, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_indegree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let indegree_centralities = centrality::indegree_centralities(&g.adj_list);
                let max_indegree_centralities = centrality::indegree_centralities(
                    &centrality::create_star_graph_undirected(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &indegree_centralities,
                    &max_indegree_centralities,
                );
                (date, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_outdegree_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let outdegree_centralities = centrality::outdegree_centralities(&g.adj_list);
                let max_outdegree_centralities = centrality::outdegree_centralities(
                    &centrality::create_star_graph_out(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &outdegree_centralities,
                    &max_outdegree_centralities,
                );
                (date, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_betweenness_centralization(filename: &str, graphs: &Graphs) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let betweenness_centralities = centrality::betweenness_centralities(&g.adj_list);
                let max_betweenness_centralities = centrality::betweenness_centralities(
                    &centrality::create_star_graph_undirected(g.adj_list.len()),
                );

                let freeman_centralization = centrality::freeman_centralization(
                    &betweenness_centralities,
                    &max_betweenness_centralities,
                );

                (date, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}
