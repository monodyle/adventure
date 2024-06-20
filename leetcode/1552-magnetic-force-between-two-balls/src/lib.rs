pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort_unstable();

    let (mut left, mut right) = (0, position[position.len() - 1] - position[0]);

    while left < right {
        let mid = (left + right + 1) / 2;
        if can_distribute(&position, mid) >= m {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    left
}

fn can_distribute(position: &Vec<i32>, distance: i32) -> i32 {
    let (mut cnt, mut last) = (1, position[0]);
    for &p in position.iter() {
        if p - last >= distance {
            cnt += 1;
            last = p;
        }
    }
    cnt
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
