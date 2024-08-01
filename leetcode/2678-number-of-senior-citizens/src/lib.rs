pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .iter()
        .filter(|p| p[11..13].parse::<i32>().unwrap_or(0) > 60)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = count_seniors(vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = count_seniors(vec![
            "1313579440F2036".to_string(),
            "2921522980M5644".to_string(),
        ]);
        assert_eq!(result, 0);
    }
}
