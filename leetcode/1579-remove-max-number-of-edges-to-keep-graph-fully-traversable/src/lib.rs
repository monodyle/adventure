pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut alice = (0..=n as usize).collect::<Vec<usize>>();
    let mut bob = (0..=n as usize).collect::<Vec<usize>>();

    fn uf_find(i: usize, uf: &mut [usize]) -> usize {
        if uf[i] != i {
            uf[i] = uf_find(uf[i], uf);
        }
        uf[i]
    }
    fn uf_union(i: usize, j: usize, uf: &mut [usize]) -> bool {
        let i = uf_find(i, uf);
        let j = uf_find(j, uf);
        if i == j {
            false
        } else {
            uf[i] = j;
            true
        }
    }

    let mut ans = 0;
    let (mut a_count, mut b_count) = (1, 1);
    for edge in &edges {
        if edge[0] == 3 {
            let a_change = uf_union(edge[1] as usize, edge[2] as usize, &mut alice);
            let b_change = uf_union(edge[1] as usize, edge[2] as usize, &mut bob);
            if a_change {
                a_count += 1;
            }
            if b_change {
                b_count += 1;
            }
            if !a_change && !b_change {
                ans += 1;
            }
        }
    }
    for edge in &edges {
        if edge[0] == 1 {
            if uf_union(edge[1] as usize, edge[2] as usize, &mut alice) {
                a_count += 1;
            } else {
                ans += 1;
            }
        } else if edge[0] == 2 {
            if uf_union(edge[1] as usize, edge[2] as usize, &mut bob) {
                b_count += 1;
            } else {
                ans += 1;
            }
        }
    }

    if a_count < n || b_count < n {
        -1
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = max_num_edges_to_remove(
            4,
            vec![
                vec![3, 1, 2],
                vec![3, 2, 3],
                vec![1, 1, 3],
                vec![1, 2, 4],
                vec![1, 1, 2],
                vec![2, 3, 4],
            ],
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn example_2() {
        let result = max_num_edges_to_remove(
            4,
            vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]],
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let result = max_num_edges_to_remove(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]);
        assert_eq!(result, -1);
    }
}
