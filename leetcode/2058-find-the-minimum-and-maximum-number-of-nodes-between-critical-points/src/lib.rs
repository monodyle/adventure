// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut min = 999999;
    let (mut first, mut cur, mut prev) = (-1, -1, -1);

    let mut root = &head;
    let mut i = 0;

    while root.is_some() {
        let val = root.as_ref().unwrap().val;
        let next = &root.as_ref().unwrap().next;
        if next.is_none() {
            break;
        }
        let next_val = next.as_ref().unwrap().val;
        if prev != -1 && (prev < val && next_val < val) || (prev > val && next_val > val) {
            if first == -1 {
                first = i;
            } else {
                min = min.min(i - cur);
            }
            cur = i;
        }
        prev = val;
        root = &next;
        i += 1;
    }

    if min == 999999 {
        vec![-1, -1]
    } else {
        vec![min, cur - first]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &val in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        head
    }

    #[test]
    fn example_1() {
        let result = nodes_between_critical_points(transform(vec![3, 1]));
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    fn example_2() {
        let result = nodes_between_critical_points(transform(vec![5, 3, 1, 2, 5, 1, 2]));
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn example_3() {
        let result = nodes_between_critical_points(transform(vec![1, 3, 2, 2, 3, 2, 2, 2, 7]));
        assert_eq!(result, vec![3, 3]);
    }
}
