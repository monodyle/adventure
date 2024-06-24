pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
    let mut min = 0;
    let mut cur = 0;

    while cur + k as usize <= nums.len() {
        if nums[cur] == 0 {
            for i in cur..cur + k as usize {
                nums[i] = 1 - nums[i]
            }
            min += 1;
        }
        cur += 1;
    }

    if nums[cur - 1..nums.len()].iter().sum::<i32>() != k {
        return -1;
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_k_bit_flips(vec![0, 1, 0], 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = min_k_bit_flips(vec![1, 1, 0], 2);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let result = min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3);
        assert_eq!(result, 3);
    }
}
