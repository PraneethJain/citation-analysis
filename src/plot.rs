use std::fs;

pub fn line_plot(filename: &str, x_vals: &[String], y_vals: &[usize]) {
    let mut content = String::new();
    for (a, b) in x_vals.iter().zip(y_vals.iter()) {
        content += &format!("{} {}\n", a, b);
    }
    if let Err(e) = fs::write(filename, content) {
        eprintln!("Error writing to file: {}", e);
    }
}
