mod common;
mod parser;
mod scc;

use scc::tarjan_scc;

fn main() {
    let graph = parser::parse();
    let _dates_map = parser::get_dates_map();
    let _sccs = tarjan_scc(&graph);

}
