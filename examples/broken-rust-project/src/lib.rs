pub fn discount(price: u32, percent: u32) -> u32 {
    price - (price * percent / 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_percent_off_one_thousand() {
        assert_eq!(discount(1000, 10), 900);
    }

    #[test]
    fn zero_percent_keeps_price() {
        assert_eq!(discount(500, 0), 500);
    }
}
