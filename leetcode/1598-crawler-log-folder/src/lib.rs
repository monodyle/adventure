pub fn min_operations(logs: Vec<String>) -> i32 {
    logs.iter().fold(0, |a, l| match l.as_str() {
        "../" => (a - 1).max(0),
        "./" => a,
        _ => a + 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string(),
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string(),
        ]);
        assert_eq!(result, 3);
    }
}
