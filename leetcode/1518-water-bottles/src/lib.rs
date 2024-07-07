pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut total = num_bottles;
    while num_bottles >= num_exchange {
        total += num_bottles / num_exchange;
        num_bottles = num_bottles / num_exchange + num_bottles % num_exchange;
    }
    total
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
