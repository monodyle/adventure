pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
    let mut map = std::collections::HashMap::<i32, i32>::new();

    for &num in &nums {
        let mapped = if num == 0 {
            mapping[0]
        } else {
            let mut num = num;
            let mut mul = 1;
            let mut new_num = 0;
            while num > 0 {
                let digit = num % 10;
                num = num / 10;
                new_num += mapping[digit as usize] * mul;
                mul *= 10;
            }
            new_num
        };
        map.insert(num, mapped);
    }
    nums.sort_by(|a, b| map.get(a).cmp(&map.get(b)));
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]);
        assert_eq!(result, vec![338, 38, 991]);
    }

    #[test]
    fn example_2() {
        let result = sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]);
        assert_eq!(result, vec![123, 456, 789]);
    }
}
