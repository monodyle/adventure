pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![i32::MAX; n+1];
    dp[0] = 0;

    for i in 0..n {
        let mut width = 0;
        let mut max_height = 0;
        for j in (0..=i).rev() {
            width += books[j][0];
            if width > shelf_width {
                break;
            }
            max_height = max_height.max(books[j][1]);
            dp[i+1] = dp[i+1].min(dp[j]+max_height)
        }
    }

    dp[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_height_shelves(vec![vec![1,1],vec![2,3],vec![2,3],vec![1,1],vec![1,1],vec![1,1],vec![1,2]], 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_2() {
        let result = min_height_shelves(vec![vec![1,3],vec![2,4],vec![3,2]], 6);
        assert_eq!(result, 4);
    }
}
