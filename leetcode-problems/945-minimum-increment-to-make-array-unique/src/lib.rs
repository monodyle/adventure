pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    /* This is fking magic guys! */
    if nums.is_empty() {
        return 0;
    }

    nums.sort_unstable();

    let mut m = 0;

    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff <= 0 {
            nums[i + 1] += 1 - diff;
            m += 1 - diff;
        }
    }

    m
}

pub fn min_increment_for_unique_2(mut nums: Vec<i32>) -> i32 {
    /* This is fking magic guys! */
    if nums.is_empty() {
        return 0;
    }

    nums.sort_unstable();

    let mut last = nums[0] - 1;
    nums.iter()
        .map(|&n| {
            if n > last {
                last = n;
                0
            } else {
                last += 1;
                last - n
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_increment_for_unique(vec![1, 2, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let result = min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_1_2() {
        let result = min_increment_for_unique_2(vec![1, 2, 2]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2_2() {
        let result = min_increment_for_unique_2(vec![3, 2, 1, 2, 1, 7]);
        assert_eq!(result, 6);
    }
}
