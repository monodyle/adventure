pub fn number_of_subarrays_sliding(nums: Vec<i32>, k: i32) -> i32 {
    let (mut odd, mut cnt, mut cur) = (0, 0, 0);

    nums.iter().fold(0, |acc, &n| {
        if n % 2 == 1 {
            odd += 1;
            cnt = 0;
        }

        while odd == k {
            if &nums[cur] % 2 == 1 {
                odd -= 1;
            }
            cnt += 1;
            cur += 1;
        }

        acc + cnt
    })
}

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    if k as usize > nums.len() {
        return 0;
    }

    let mut nice = 0;
    let mut odd = 0;
    let mut count = vec![0; nums.len() + 1];
    count[0] = 1; // a subarray starting from index 0 is considered

    for n in nums {
        if n % 2 == 1 {
            odd += 1;
        }
        if odd >= k {
            nice += count[(odd - k) as usize]
        }
        count[odd as usize] += 1;
    }

    nice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = number_of_subarrays(vec![1, 1, 2, 1, 1], 3);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = number_of_subarrays(vec![2, 4, 6], 6);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let result = number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2);
        assert_eq!(result, 16);
    }
}
