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

pub struct Solution {}


impl Solution {
    fn to_sorted_stack(node: Box<ListNode>, sorted_stack: &mut Vec<Box<ListNode>>) {
        const MAX: i32 = 10_000;
        let res = sorted_stack.binary_search_by_key(&(MAX - node.val), |b| MAX - b.val);
        match res {
            Ok(idx) => sorted_stack.insert(idx, node),
            Err(idx) => sorted_stack.insert(idx, node),
        }
    }

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 || lists.len() == 1 && lists[0].is_none() {
            return None;
        }

        let mut stack: Vec<Box<ListNode>> = Vec::from([]);

        for row in lists {
            let node = row.unwrap();
            Self::to_sorted_stack(node, &mut stack);
        }

        // let mut out = ListNode::new(0);
        let mut next = Box::new(ListNode::new(0));

        let mut curr_node = &mut next;

        while stack.len() > 0 {
            let value = *stack.pop().unwrap();
            println!("- {}", value.val);
            let new_node = Box::new(ListNode::new(value.val));
            curr_node.next = Some(new_node);
            curr_node = curr_node.next.as_mut().unwrap();
            // swap(&mut next, &mut new);
            // out.next = Some(Box::new(next));
            if value.next.is_some() {
              Self::to_sorted_stack(value.next.unwrap(), &mut stack);
            }
        }
        next.next
    }
}
