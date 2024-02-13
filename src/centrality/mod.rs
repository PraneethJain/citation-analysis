pub mod degree_centrality;

pub use degree_centrality::degree_centralities;

pub fn freeman_centralization(centralities: &[usize], theoretical_maximum: usize) -> f32 {
    let max = centralities.iter().max().unwrap();
    centralities.iter().map(|c| max - c).sum::<usize>() as f32 / theoretical_maximum as f32
}
