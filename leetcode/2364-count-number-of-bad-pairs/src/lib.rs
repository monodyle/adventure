pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
    let mut counts = std::collections::HashMap::new();
    let mut bad = 0;

    for i in 0..nums.len() {
        let good = counts.entry(i as i32 - nums[i]).or_insert(0);
        bad += i - *good;
        *good += 1;
    }

    bad as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let example_1 = count_bad_pairs(vec![4, 1, 3, 3]);
        assert_eq!(example_1, 5);

        let example_2 = count_bad_pairs(vec![1, 2, 3, 4, 5]);
        assert_eq!(example_2, 0);
    }
}
