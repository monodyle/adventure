pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut total = 0;
    let mut filled = num_bottles;
    let mut empty = 0;
    while filled + empty >= num_exchange {
        total += filled;
        empty += filled;
        filled = empty / num_exchange;
        empty -= num_exchange * (empty / num_exchange);
    }
    total + filled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = num_water_bottles(9, 3);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let result = num_water_bottles(15, 4);
        assert_eq!(result, 19);
    }
}
