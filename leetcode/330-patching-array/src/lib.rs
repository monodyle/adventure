pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let mut patchs = 0;
    let mut sum: i64 = 1;
    let mut i = 0;

    while sum <= n as i64 {
        if i < nums.len() && sum >= nums[i] as i64 {
            sum += nums[i] as i64;
            i += 1
        } else {
            sum += sum;
            patchs += 1
        }
    }

    patchs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_patches(vec![1, 3], 6);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let result = min_patches(vec![1, 5, 10], 20);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_3() {
        let result = min_patches(vec![1, 2, 2], 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_4() {
        let result = min_patches(vec![1,2,31,33], 2147483647);
        assert_eq!(result, 0);
    }
}
