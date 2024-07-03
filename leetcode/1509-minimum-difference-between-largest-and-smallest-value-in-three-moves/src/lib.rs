pub fn min_difference(nums: Vec<i32>) -> i32 {
    if nums.len() < 5 {
        return 0;
    }

    let (mut largest, mut smallest) = (vec![i32::MIN; 4], vec![i32::MAX; 4]);

    for &num in &nums {
        for i in 0..4 {
            if num > largest[i] {
                for j in (i + 1..4).rev() {
                    largest[j] = largest[j - 1];
                }
                largest[i] = num;
                break;
            }
        }

        for i in 0..4 {
            if num < smallest[i] {
                for j in (i + 1..4).rev() {
                    smallest[j] = smallest[j - 1];
                }
                smallest[i] = num;
                break;
            }
        }
    }

    largest
        .into_iter()
        .zip(smallest.into_iter().rev())
        .map(|(large, small)| large - small)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_difference(vec![5, 3, 2, 4]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_2() {
        let result = min_difference(vec![1, 5, 0, 10, 14]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let result = min_difference(vec![3, 100, 20]);
        assert_eq!(result, 0);
    }
}
