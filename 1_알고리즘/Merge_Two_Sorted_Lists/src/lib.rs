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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => {
            return None;
        }
        (Some(node), None) | (None, Some(node)) => {
            return Some(node);
        }
        (Some(node1), Some(node2)) => {
            if node1.val >= node2.val {
                return Some(Box::new(ListNode {
                    val: node2.val,
                    next: merge_two_lists(Some(node1), node2.next),
                }));
            } else {
                return Some(Box::new(ListNode {
                    val: node1.val,
                    next: merge_two_lists(node1.next, Some(node2)),
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut one = Box::new(ListNode::new(1));
        let mut two = Box::new(ListNode::new(2));
        let mut four = Box::new(ListNode::new(4));

        two.next = Some(four);
        one.next = Some(two);

        let mut three = Box::new(ListNode::new(3));
        let mut five = Box::new(ListNode::new(5));

        three.next = Some(five);

        let mut head = merge_two_lists(Some(one), Some(three));
        let mut answer = vec![];

        while head.is_some() {
            let node = head.unwrap();
            answer.push(node.val);
            head = node.next;
        }

        assert_eq!(answer, vec![1, 2, 3, 4, 5]);
    }
}
