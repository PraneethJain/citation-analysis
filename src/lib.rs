pub mod centrality;
pub mod common;
pub mod community_detection;
pub mod graphviz;
pub mod linear_regression;
pub mod parser;
pub mod plot;

pub use common::*;

use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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

pub fn save_louvain_graphs(graphs: &Graphs) {
    dates().for_each(|date| {
        let g = graphs.till(&date);
        let mut g_undirected = g.clone();
        for (from, tos) in g.adj_list.iter().enumerate() {
            for &to in tos {
                g_undirected.adj_list[to].push(from);
            }
        }
        let largest_scc_graph = community_detection::largest_scc_graph(&g_undirected);
        let mut seen = BTreeSet::new();

        let mut content = String::from("source,target\n");
        for (from, tos) in largest_scc_graph.adj_list.iter().enumerate() {
            for &to in tos {
                if seen.insert((to, from)) && seen.insert((from, to)) {
                    content += &format!("{},{}\n", from, to);
                }
            }
        }
        if let Err(e) = fs::write(
            &format!(
                "community_graphs/louvain/{:04}-{:02}.txt",
                date.year, date.month
            ),
            content,
        ) {
            eprintln!("Error writing to file: {}", e);
        }

        let output = Command::new("just")
            .args([
                "rr",
                "community",
                &format!(
                    "/home/bp87/Repos/citation-analysis/community_graphs/louvain/{:04}-{:02}.txt",
                    date.year, date.month
                ),
                "-s",
                &format!(
                    "/home/bp87/Repos/citation-analysis/community_graphs/louvain/l{:04}-{:02}.txt",
                    date.year, date.month
                ),
                "-h",
                "hierarchy.tmp",
            ])
            .current_dir("/home/bp87/Repos/testing/fast-louvain")
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            let mut communities = vec![vec![]; largest_scc_graph.adj_list.len()];
            let mut max_community_index = 0;
            if let Ok(lines) = read_lines(&format!(
                "community_graphs/louvain/l{:04}-{:02}.txt",
                date.year, date.month
            )) {
                for line in lines.flatten().skip(1) {
                    let (node_index, community_index): (usize, usize) = match line.split_once(",") {
                        Some((a, b)) => (a.parse().unwrap(), b.parse().unwrap()),
                        None => panic!("no ,  in line: {:?}", line),
                    };
                    communities[community_index].push(node_index);
                    max_community_index = max_community_index.max(community_index);
                }
            }
            communities.resize(max_community_index, vec![]);

            let filename = String::from("community_graphs/louvain/g")
                + &format!("{:04}-{:02}", date.year, date.month)
                + ".gv";
            graphviz::save_with_colors(&filename, &largest_scc_graph.adj_list, &communities);
        }
    })
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
