pub fn truncate_to_ticks(price: f64, ticks: u32) -> f64 {
    let factor = 10f64.powi(ticks as i32);
    (price * factor).trunc() / factor
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_zero() {
        assert_eq!(truncate_to_ticks(5.789, 0), 5.0);
        assert_eq!(truncate_to_ticks(-5.789, 0), -5.0);
    }

    #[test]
    fn test_truncate_to_one() {
        assert_eq!(truncate_to_ticks(5.789, 1), 5.7);
        assert_eq!(truncate_to_ticks(-5.789, 1), -5.7);
    }

    #[test]
    fn test_truncate_to_two() {
        assert_eq!(truncate_to_ticks(5.789, 2), 5.78);
        assert_eq!(truncate_to_ticks(-5.789, 2), -5.78);
    }

    #[test]
    fn test_truncate_to_three() {
        assert_eq!(truncate_to_ticks(5.789, 3), 5.789);
        assert_eq!(truncate_to_ticks(-5.789, 3), -5.789);
    }

    #[test]
    fn test_truncate_exact() {
        // Numbers that already have exact decimal places
        assert_eq!(truncate_to_ticks(5.7, 1), 5.7);
        assert_eq!(truncate_to_ticks(5.78, 2), 5.78);
    }

    #[test]
    fn test_truncate_large_decimals() {
        assert_eq!(truncate_to_ticks(123.456789, 4), 123.4567);
        assert_eq!(truncate_to_ticks(-123.456789, 5), -123.45678);
    }

    #[test]
    fn test_truncate_zero() {
        assert_eq!(truncate_to_ticks(0.0, 3), 0.0);
        assert_eq!(truncate_to_ticks(-0.0, 3), -0.0);
    }
}
