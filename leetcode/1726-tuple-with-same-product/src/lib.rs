pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut map = std::collections::HashMap::<i32, i32>::with_capacity((n * (n - 1)) / 2);
    for i in 0..n {
        for j in i + 1..n {
            let product = nums[i] * nums[j];
            *map.entry(product).or_insert(0) += 1;
        }
    }

    map.values()
        .filter(|&&freq| freq > 1)
        .fold(0, |acc, &freq| acc + freq * (freq - 1) * 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = tuple_same_product(vec![2, 3, 4, 6]);
        assert_eq!(result, 8);
    }

    #[test]
    fn example_2() {
        let result = tuple_same_product(vec![1, 2, 4, 5, 10]);
        assert_eq!(result, 16);
    }
}
