pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort_unstable();

    let (mut left, mut right) = (
        0,
        (position[position.len() - 1] - position[0]) / (m - 1) + 1,
    );

    while left < right {
        let mid = (left + right + 1) / 2;
        if can_distribute(&position, m, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn can_distribute(position: &Vec<i32>, mut m: i32, distance: i32) -> bool {
    let mut last = -distance;
    for &p in position {
        if p - last >= distance {
            m -= 1;
            last = p;
        }
        if m == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = max_distance(vec![1, 2, 3, 4, 7], 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2);
        assert_eq!(result, 999999999);
    }
}
