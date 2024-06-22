pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut ans = std::usize::MAX;
    let mut sum = 0;
    let mut left = 0;

    for right in 0..nums.len() {
        sum += nums[right];
        if sum >= target {
            while sum >= target {
                ans = ans.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }
    }

    if ans == std::usize::MAX {
        0
    } else {
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = min_sub_array_len(4, vec![1, 4, 4]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let result = min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(result, 0);
    }
}
