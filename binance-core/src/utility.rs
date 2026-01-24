pub fn truncate_to_ticks(price: f64, ticks: u32) -> f64 {
    let factor = 10f64.powi(ticks as i32);
    (price * factor).trunc() / factor
}
