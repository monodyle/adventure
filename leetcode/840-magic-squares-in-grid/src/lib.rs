pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if grid[i][j] == 5
                && grid[i - 1][j - 1] < 10
                && grid[i - 1][j - 1] > 0
                && grid[i - 1][j] < 10
                && grid[i - 1][j] > 0
                && grid[i - 1][j + 1] < 10
                && grid[i - 1][j + 1] > 0
                && grid[i][j - 1] < 10
                && grid[i][j - 1] > 0
                && grid[i][j + 1] < 10
                && grid[i][j + 1] > 0
                && grid[i + 1][j - 1] < 10
                && grid[i + 1][j - 1] > 0
                && grid[i + 1][j] < 10
                && grid[i + 1][j] > 0
                && grid[i + 1][j + 1] < 10
                && grid[i + 1][j + 1] > 0
                && (grid[i - 1][j - 1]
                    + grid[i - 1][j]
                    + grid[i - 1][j + 1]
                    + grid[i][j - 1]
                    + grid[i][j]
                    + grid[i][j + 1]
                    + grid[i + 1][j - 1]
                    + grid[i + 1][j]
                    + grid[i + 1][j + 1])
                    == 45
                && grid[i - 1][j - 1] != grid[i][j]
                && grid[i - 1][j] != grid[i][j]
                && grid[i - 1][j + 1] != grid[i][j]
                && grid[i][j - 1] != grid[i][j]
                && grid[i][j + 1] != grid[i][j]
                && grid[i + 1][j - 1] != grid[i][j]
                && grid[i + 1][j] != grid[i][j]
                && grid[i + 1][j + 1] != grid[i][j]
                && grid[i - 1][j] + grid[i + 1][j] == 10
                && grid[i][j - 1] + grid[i][j + 1] == 10
                && grid[i - 1][j - 1] + grid[i + 1][j + 1] == 10
                && grid[i - 1][j + 1] + grid[i + 1][j - 1] == 10
                && grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1] == 15
                && grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1] == 15
                && grid[i - 1][j - 1] + grid[i][j - 1] + grid[i + 1][j - 1] == 15
                && grid[i - 1][j + 1] + grid[i][j + 1] + grid[i + 1][j + 1] == 15
            {
                ans += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result =
            num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]]);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_2() {
        let result = num_magic_squares_inside(vec![vec![8]]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_10() {
        let result = num_magic_squares_inside(vec![vec![5,5,5],vec![5,5,5],vec![5,5,5]]);
        assert_eq!(result, 0);
    }
}
