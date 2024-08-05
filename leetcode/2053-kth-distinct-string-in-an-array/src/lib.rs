pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let mut map = std::collections::HashMap::new();
    for s in arr.iter() {
        *map.entry(s).or_insert(0) += 1;
    }

    arr.iter()
        .filter(|&s| map[s] == 1)
        .nth((k - 1) as usize)
        .unwrap_or(&"".to_string())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = kth_distinct(
            vec![
                "d".to_string(),
                "b".to_string(),
                "c".to_string(),
                "b".to_string(),
                "c".to_string(),
                "a".to_string(),
            ],
            2,
        );
        assert_eq!(result, "a");
    }

    #[test]
    fn example_2() {
        let result = kth_distinct(
            vec!["aaa".to_string(), "aa".to_string(), "a".to_string()],
            1,
        );
        assert_eq!(result, "aaa");
    }

    #[test]
    fn example_3() {
        let result = kth_distinct(vec!["a".to_string(), "b".to_string(), "a".to_string()], 3);
        assert_eq!(result, "");
    }
}
