pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    if (m * k) as usize > bloom_day.len() {
        return -1;
    }

    let can_bloom = |day: i32| {
        let (mut prev, mut ans) = (0, 0);
        for i in 0..bloom_day.len() {
            if bloom_day[i] <= day {
                if 1 + i - prev == k as usize {
                    prev = i + 1;
                    ans += 1;
                }
            } else {
                prev = i + 1;
            }

            if ans == m {
                return true;
            }
        }

        false
    };

    let dday = *bloom_day.iter().max().unwrap();
    let (mut min, mut max) = (0, dday + 1);

    while min < max {
        let mid = (min + max) / 2;
        if can_bloom(mid) {
            max = mid;
        } else {
            min = mid + 1;
        }
    }

    if min > dday {
        -1
    } else {
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_days(vec![1, 10, 3, 10, 2], 3, 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = min_days(vec![1, 10, 3, 10, 2], 3, 2);
        assert_eq!(result, -1);
    }

    #[test]
    fn example_3() {
        let result = min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3);
        assert_eq!(result, 12);
    }
}
