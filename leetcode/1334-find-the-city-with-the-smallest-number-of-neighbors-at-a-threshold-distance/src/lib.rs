pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let adjacency_list = {
        let mut res = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            res[u].push((v, w));
            res[v].push((u, w));
        }
        res
    };

    let mut final_results = vec![];
    for end in 0..n {
        let mut distances = vec![i32::MAX; n];
        distances[end] = 0;
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((std::cmp::Reverse(0), end));
        while let Some((std::cmp::Reverse(distance), u)) = pq.pop() {
            for &(v, w) in &adjacency_list[u] {
                let next_distance = distance + w;
                if next_distance < distances[v] {
                    distances[v] = next_distance;
                    pq.push((std::cmp::Reverse(next_distance), v));
                }
            }
        }
        let count = distances
            .into_iter()
            .filter(|&a| a > 0 && a <= distance_threshold)
            .count();
        final_results.push((count, std::cmp::Reverse(end)));
    }

    final_results.sort();
    (final_results.first().unwrap().1).0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_the_city(
            4,
            vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
            4,
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = find_the_city(
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1],
            ],
            2,
        );
        assert_eq!(result, 0);
    }
}
