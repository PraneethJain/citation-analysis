pub fn linear_regression(x: &[usize], y: &[usize]) -> (f32, f32) {
    let n = x.len() as f32;

    let m_x = x.iter().sum::<usize>() as f32 / n;
    let m_y = y.iter().sum::<usize>() as f32 / n;

    let ss_xy = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum::<usize>() as f32 - n * m_y * m_x;
    let ss_xx = x.iter().zip(x.iter()).map(|(a, b)| a * b).sum::<usize>() as f32 - n * m_x * m_x;

    let m = ss_xy / ss_xx;
    let c = m_y - m * m_x;

    (m, c)
}
