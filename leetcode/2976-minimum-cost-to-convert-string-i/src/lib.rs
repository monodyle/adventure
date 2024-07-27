pub fn minimum_cost(
    source: String,
    target: String,
    original: Vec<char>,
    changed: Vec<char>,
    cost: Vec<i32>,
) -> i64 {
    let mut c = vec![vec![i32::MAX as u64; 26]; 26];
    for i in 0..original.len() {
        let c1 = original[i] as usize - 97;
        let c2 = changed[i] as usize - 97;
        c[c1][c2] = c[c1][c2].min(cost[i] as u64);
    }

    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                c[i][j] = c[i][j].min(c[i][k] + c[k][j]);
            }
        }
    }

    let mut ans = 0;
    let source = source.into_bytes();
    let target = target.into_bytes();

    for i in 0..source.len() {
        if target[i] != source[i] {
            let v = c[source[i] as usize - 97][target[i] as usize - 97];
            if v >= i32::MAX as u64 {
                return -1
            }
            ans += v
        }
    }

    ans as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20],
        );
        assert_eq!(result, 28);
    }

    #[test]
    fn example_2() {
        let result = minimum_cost(
            "aaaa".to_string(),
            "bbbb".to_string(),
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2],
        );
        assert_eq!(result, 12);
    }

    #[test]
    fn example_3() {
        let result = minimum_cost(
            "abcd".to_string(),
            "abce".to_string(),
            vec!['a'],
            vec!['e'],
            vec![10000],
        );
        assert_eq!(result, -1);
    }
}
