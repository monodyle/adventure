pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    if let Ok(_) = nums.binary_search(&target) {
        vec![
            nums.partition_point(|n| n < &target) as i32,
            nums.partition_point(|n| n <= &target) as i32 - 1,
        ]
    } else {
        vec![-1, -1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn example_2() {
        let result = search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn example_3() {
        let result = search_range(vec![], 0);
        assert_eq!(result, vec![-1, -1]);
    }
}
