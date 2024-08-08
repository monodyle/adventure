pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    std::iter::once((0, 0))
        .chain((0..).flat_map(|i| {
            let first_leg = (2 * i + 1) as usize;
            std::iter::repeat((0, 1))
                .take(first_leg)
                .chain(std::iter::repeat((1, 0)).take(first_leg))
                .chain(std::iter::repeat((0, -1)).take(first_leg + 1))
                .chain(std::iter::repeat((-1, 0)).take(first_leg + 1))
        }))
        .scan((r_start, c_start), |(ref mut x, ref mut y), (dx, dy)| {
            *x += dx;
            *y += dy;
            Some((*x, *y))
        })
        .filter(|(ref x, ref y)| 0 <= *x && *x < rows && 0 <= *y && *y < cols)
        .take((rows * cols) as usize)
        .map(|(x, y)| vec![x, y])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = spiral_matrix_iii(1, 4, 0, 0);
        assert_eq!(result, [[0, 0], [0, 1], [0, 2], [0, 3]]);
    }

    #[test]
    fn example_2() {
        let result = spiral_matrix_iii(5, 6, 1, 4);
        assert_eq!(
            result,
            [
                [1, 4],
                [1, 5],
                [2, 5],
                [2, 4],
                [2, 3],
                [1, 3],
                [0, 3],
                [0, 4],
                [0, 5],
                [3, 5],
                [3, 4],
                [3, 3],
                [3, 2],
                [2, 2],
                [1, 2],
                [0, 2],
                [4, 5],
                [4, 4],
                [4, 3],
                [4, 2],
                [4, 1],
                [3, 1],
                [2, 1],
                [1, 1],
                [0, 1],
                [4, 0],
                [3, 0],
                [2, 0],
                [1, 0],
                [0, 0]
            ]
        );
    }
}
