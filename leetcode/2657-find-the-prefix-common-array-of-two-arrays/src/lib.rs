pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let mut seen = std::collections::HashSet::<i32>::new();
    let mut count = 0;
    for i in 0..a.len() {
        if seen.contains(&a[i]) {
            count += 1
        }
        if seen.contains(&b[i]) {
            count += 1
        }
        if a[i] == b[i] {
            count += 1
        }
        result.push(count);
        seen.insert(a[i]);
        seen.insert(b[i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]);
        assert_eq!(result, vec![0, 2, 3, 4]);
    }

    #[test]
    fn example_2() {
        let result = find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]);
        assert_eq!(result, vec![0, 1, 3]);
    }
}
