mod common;
mod parser;
mod scc;

fn main() {
    let graph = parser::parse();
    println!("{:?}", graph);
    let _dates_map = parser::get_dates_map();
}
