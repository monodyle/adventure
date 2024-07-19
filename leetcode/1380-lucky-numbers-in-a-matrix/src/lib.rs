pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mins: std::collections::HashSet<_> =
        matrix.iter().map(|r| *r.iter().min().unwrap()).collect();
    let maxs: std::collections::HashSet<_> = (0..matrix[0].len())
        .map(|i| matrix.iter().map(|r| r[i]).max().unwrap())
        .collect();
    mins.intersection(&maxs).map(|&n| n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]);
        assert_eq!(result, vec![15]);
    }

    #[test]
    fn example_2() {
        let result = lucky_numbers(vec![
            vec![1, 10, 4, 2],
            vec![9, 3, 8, 7],
            vec![15, 16, 17, 12],
        ]);
        assert_eq!(result, vec![12]);
    }

    #[test]
    fn example_3() {
        let result = lucky_numbers(vec![vec![7, 8], vec![1, 2]]);
        assert_eq!(result, vec![7]);
    }
}
