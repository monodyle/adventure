pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .bytes()
        .map(|c| c as i32 - 65 + 1)
        .fold(0, |acc, c| acc * 26 + c as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = title_to_number("A".to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let result = title_to_number("AB".to_string());
        assert_eq!(result, 28);
    }

    #[test]
    fn example_3() {
        let result = title_to_number("ZY".to_string());
        assert_eq!(result, 701);
    }
}
