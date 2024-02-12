mod common;
mod parser;
mod scc;

fn main() {
    let _graph = parser::parse();
    let _dates_map = parser::get_dates_map();
}
