pub mod centrality;
pub mod common;
pub mod community_detection;
pub mod graphviz;
pub mod linear_regression;
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

fn save_freeman_centralization(
    filename: &str,
    graphs: &Graphs,
    centralities_func: fn(&Vec<Vec<usize>>) -> Vec<f32>,
    max_graph_func: fn(usize) -> Vec<Vec<usize>>,
) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                let centralities = centralities_func(&g.adj_list);
                let max_centralities = centralities_func(&max_graph_func(g.adj_list.len()));
                let freeman_centralization =
                    centrality::freeman_centralization(&centralities, &max_centralities);
                (date, freeman_centralization)
            })
            .collect::<Vec<_>>(),
    )
}

fn save_max_centrality(
    filename: &str,
    graphs: &Graphs,
    centralities_func: fn(&Vec<Vec<usize>>) -> Vec<f32>,
) {
    plot::line_plot(
        filename,
        &dates()
            .map(|date| {
                let g = graphs.till(&date);
                (
                    date,
                    centralities_func(&g.adj_list)
                        .into_iter()
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>(),
    )
}

pub fn save_freeman_degree_centralization(filename: &str, graphs: &Graphs) {
    save_freeman_centralization(
        filename,
        graphs,
        centrality::degree_centralities,
        centrality::create_star_graph_undirected,
    )
}

pub fn save_max_degree_centralization(filename: &str, graphs: &Graphs) {
    save_max_centrality(filename, graphs, centrality::degree_centralities)
}

pub fn save_freeman_indegree_centralization(filename: &str, graphs: &Graphs) {
    save_freeman_centralization(
        filename,
        graphs,
        centrality::indegree_centralities,
        centrality::create_star_graph_in,
    )
}

pub fn save_max_indegree_centralization(filename: &str, graphs: &Graphs) {
    save_max_centrality(filename, graphs, centrality::indegree_centralities)
}

pub fn save_freeman_outdegree_centralization(filename: &str, graphs: &Graphs) {
    save_freeman_centralization(
        filename,
        graphs,
        centrality::outdegree_centralities,
        centrality::create_star_graph_out,
    )
}

pub fn save_max_outdegree_centralization(filename: &str, graphs: &Graphs) {
    save_max_centrality(filename, graphs, centrality::outdegree_centralities)
}

pub fn save_freeman_betweenness_centralization(filename: &str, graphs: &Graphs) {
    save_freeman_centralization(
        filename,
        graphs,
        centrality::betweenness_centralities,
        centrality::create_star_graph_undirected,
    )
}

pub fn save_max_betweenness_centralization(filename: &str, graphs: &Graphs) {
    save_max_centrality(filename, graphs, centrality::betweenness_centralities)
}

pub fn save_linear_regression_largest_wcc_sizes(filename: &str, graphs: &Graphs) -> f32 {
    let largest_wcc_sizes = &dates()
        .filter_map(|date| {
            if date <= Date::new(2002, 1, 1) {
                let g = graphs.till(&date);
                let scc = community_detection::tarjan_scc(&g.adj_list);
                Some(scc.iter().map(|x| x.len()).max().unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let (m, c) = linear_regression::linear_regression(
        &(0..largest_wcc_sizes.len()).collect::<Vec<_>>(),
        largest_wcc_sizes,
    );

    plot::line_plot(
        filename,
        &dates()
            .enumerate()
            .map(|(i, date)| (date, m * i as f32 + c))
            .collect::<Vec<_>>(),
    );

    let mean = largest_wcc_sizes.iter().sum::<usize>() as f32 / largest_wcc_sizes.len() as f32;
    let mut rss = 0.0;
    let mut tss = 0.0;
    for (i, &size) in largest_wcc_sizes.into_iter().enumerate() {
        rss += (size as f32 - (m * i as f32 + c)).powf(2.0);
        tss += (size as f32 - mean).powf(2.0);
    }
    1.0 - rss / tss
}
