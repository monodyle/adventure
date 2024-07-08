pub fn find_the_winner(n: i32, k: i32) -> i32 {
    (2..=n).fold(0, |s, i| (s+k)%i)+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_the_winner(5, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = find_the_winner(6, 5);
        assert_eq!(result, 1);
    }
}
