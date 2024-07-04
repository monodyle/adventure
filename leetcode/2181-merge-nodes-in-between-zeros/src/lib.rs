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

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut merged = vec![];
    let mut sum = 0;
    let mut cur: Option<Box<ListNode>> = head;
    while let Some(node) = cur {
        if node.val == 0 && sum != 0 {
            merged.push(sum);
            sum = 0;
        } else {
            sum += node.val;
        }
        cur = node.next;
    }

    let mut ans: Option<Box<ListNode>> = None;
    for &val in merged.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = ans;
        ans = Some(node);
    }
    ans
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
        let input = transform(vec![0, 3, 1, 0, 4, 5, 2, 0]);
        let output = transform(vec![4, 11]);
        let result = merge_nodes(input);
        assert_eq!(result, output);
    }

    #[test]
    fn example_2() {
        let input = transform(vec![0, 1, 0, 3, 0, 2, 2, 0]);
        let output = transform(vec![1, 3, 4]);
        let result = merge_nodes(input);
        assert_eq!(result, output);
    }
}
