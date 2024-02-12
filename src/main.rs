mod common;
mod parser;
mod scc;

use scc::StronglyConnectedComponents;

fn main() {
    let graph = parser::parse();
    println!("{:?}", graph);
    let _dates_map = parser::get_dates_map();

    let mut scc = StronglyConnectedComponents::new(graph.len());
    scc.find_components(&graph);

    println!("{}", scc.num_components);
}
