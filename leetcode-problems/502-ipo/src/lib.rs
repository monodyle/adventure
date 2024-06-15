use std::collections::BinaryHeap;

pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    if k == 0 {
        return w;
    }

    let mut projects = capital
        .into_iter()
        .zip(profits.into_iter())
        .collect::<Vec<(i32, i32)>>();
    projects.sort_unstable_by_key(|k| k.0);

    let mut heap = BinaryHeap::with_capacity(projects.len());
    let mut next = 0;

    for _ in 0..k {
        while let Some(pj) = projects.get(next).filter(|p| p.0 <= w) {
            heap.push(pj.1);
            next += 1;
        }
        if let Some(pf) = heap.pop() {
            w += pf;
        }
    }

    /* while let Some(pj) = projects.pop() {
        if pj.0 <= w {
            heap.push(pj.1);
        } else {
            projects.push(pj);
            break;
        }
    }

    while let Some(h) = heap.pop() {
        if k == 0 {
            break;
        }

        w += h;
        k -= 1;

        while let Some(pj) = projects.pop() {
            if pj.0 <= w {
                heap.push(pj.1);
            } else {
                projects.push(pj);
                break;
            }
        }
    } */

    w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]);
        assert_eq!(result, 6);
    }
}
