pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut max = nums[left];
    for i in 1..nums.len() {
        if nums[i] < max {
            left = i + 1;
        } else {
            max = nums[i];
        }
    }

    if left == 0 {
        return 0;
    }

    let mut right = nums.len() - 1;
    let mut min = nums[right];
    for i in (0..right).rev() {
        if nums[i] > min {
            right = i;
        } else {
            min = nums[i];
        }
    }

    (left - right) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]);
        assert_eq!(result, 5 - 1);
    }

    #[test]
    fn example_2() {
        let result = find_unsorted_subarray(vec![1, 2, 3, 4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let result = find_unsorted_subarray(vec![1]);
        assert_eq!(result, 0);
    }
}
