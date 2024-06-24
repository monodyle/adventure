// queue
pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let (mut min, k, n) = (0, k as usize, nums.len());
    let mut queue = std::collections::VecDeque::new();

    for (bit, index) in nums.into_iter().zip(0..) {
        while queue.front().is_some_and(|&i| i <= index) {
            queue.pop_front();
        }

        if queue.len() & 1 == bit as usize {
            if index + k > n {
                return -1;
            }
            queue.push_back(index + k);
            min += 1
        }
    }

    min
}

pub fn min_k_bit_flips_2(mut nums: Vec<i32>, k: i32) -> i32 {
    let (mut min, mut cur) = (0, 0);

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
