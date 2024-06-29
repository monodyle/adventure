pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    let mut degree = vec![0; n];

    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
        degree[edge[1] as usize] += 1;
    }

    let mut ans = vec![Vec::new(); n];
    let mut queue = degree
        .iter()
        .enumerate()
        .filter_map(|(i, &degree)| (degree == 0).then(|| i))
        .collect::<std::collections::VecDeque<usize>>();

    while let Some(i) = queue.pop_front() {
        ans[i].sort_unstable();
        ans[i].dedup();
        for &j in graph[i].iter() {
            let origin = ans[i].clone();
            ans[j].extend(origin);
            ans[j].push(i as i32);
            degree[j] -= 1;
            if degree[j] == 0 {
                queue.push_back(j);
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
        let result = get_ancestors(
            8,
            vec![
                vec![0, 3],
                vec![0, 4],
                vec![1, 3],
                vec![2, 4],
                vec![2, 7],
                vec![3, 5],
                vec![3, 6],
                vec![3, 7],
                vec![4, 6],
            ],
        );
        assert_eq!(
            result,
            vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![0, 2],
                vec![0, 1, 3],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 2, 3]
            ]
        );
    }

    #[test]
    fn example_2() {
        let result = get_ancestors(
            5,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![0, 4],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ],
        );
        assert_eq!(
            result,
            vec![vec![], vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]]
        );
    }
}
