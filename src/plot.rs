use std::fmt::Display;
use std::fs;

pub fn line_plot<T, U>(filename: &str, vals: &[(T, U)])
where
    T: Display,
    U: Display,
{
    let mut content = String::new();
    vals.iter()
        .for_each(|(a, b)| content += &format!("{} {}\n", a, b));
    if let Err(e) = fs::write(filename, content) {
        eprintln!("Error writing to file: {}", e);
    }
}
