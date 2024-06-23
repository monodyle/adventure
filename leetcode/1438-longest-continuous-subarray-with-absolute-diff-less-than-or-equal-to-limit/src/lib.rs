pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut longest = 1;
    let (mut max, mut min) = (0, 0);
    let mut cur = 0;

    for i in 1..nums.len() {
        if nums[i] >= nums[max] {
            max = i;
        }
        if nums[i] <= nums[min] {
            min = i;
        }

        if nums[max] - nums[min] <= limit {
            longest = longest.max(i - cur + 1)
        } else {
            if max < min {
                max += 1;
                cur = max;
                max = (cur + 1..=i).fold(max, |mut max, new| {
                    if nums[new] >= nums[max] {
                        max = new;
                    }
                    max
                })
            } else {
                min += 1;
                cur = min;
                min = (cur + 1..=i).fold(min, |mut min, new| {
                    if nums[new] <= nums[min] {
                        min = new;
                    }
                    min
                })
            }
        }
    }

    longest as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = longest_subarray(vec![8, 2, 4, 7], 4);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = longest_subarray(vec![10, 1, 2, 4, 7, 2], 5);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_3() {
        let result = longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0);
        assert_eq!(result, 3);
    }
}
