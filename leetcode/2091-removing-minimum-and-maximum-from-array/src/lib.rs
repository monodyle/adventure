pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let (mut min, mut max) = (0, 0);
    for (i, &n) in nums.iter().enumerate() {
        if n < nums[min] {
            min = i;
        }
        if n > nums[max] {
            max = i;
        }
    }

    let lower = min.min(max);
    let higher = max.max(min);

    let mut ans = (higher + 1).min(nums.len() - lower);
    ans = ans.min(lower + 1 + nums.len() - higher);

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6]);
        assert_eq!(result, 5);
    }

    #[test]
    fn example_2() {
        let result = minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let result = minimum_deletions(vec![101]);
        assert_eq!(result, 1);
    }
}
