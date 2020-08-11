/*
Merge two sorted linked lists and return it as a new list.
The new list should be made by splicing together the nodes of the first two lists.

Example:

Input: 1->2->4, 1->3->4
Output: 1->1->2->3->4->4
*/
pub struct Solution {}

#[path = "../../../util/linked_list.rs"]
mod linked_list;
use linked_list::{from_vec, ListNode};

/* simple iterative solution
use std::mem;
impl Solution {
  pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut result = l1;
    let mut l2 = l2;
    let mut lsmall = &mut result;
    let lbig = &mut l2;
    while lbig.is_some() {
      if lsmall.is_none() || lsmall.as_ref()?.val > lbig.as_ref()?.val {
        mem::swap(lsmall, lbig);
      }
      if lsmall.is_some() {
        lsmall = &mut lsmall.as_mut()?.next;
      }
    }
    result
  }
}
*/

/* recursive solution
impl Solution {
  pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let some_box_list = |val, next| Some(Box::new(ListNode { val, next }));
    match (l1, l2) {
      (Some(left_node), Some(right_node)) => if left_node.val < right_node.val {
          some_box_list(left_node.val, Self::merge_two_lists(left_node.next, Some(right_node)))
      } else {
          some_box_list(right_node.val, Self::merge_two_lists(Some(left_node), right_node.next))
      },
      (Some(left_node), _) => some_box_list(left_node.val, Self::merge_two_lists(left_node.next, None)),
      (l1, l2 @ Some(_)) => Self::merge_two_lists(l2, l1),
      _ => None
    }
  }
}
*/

// easily understandable iterative solution
impl Solution {
  pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let get_next = |mut l: Option<Box<ListNode>>| (l.as_mut().unwrap().next.take(), l.take());
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
    let mut head = &mut dummy_head;
    let (mut l1, mut l2) = (l1, l2);
    while l1.is_some() || l2.is_some() {
      if l1.is_none() {
        head.as_mut()?.next = l2;
        break;
      } else if l2.is_none() {
        head.as_mut()?.next = l1;
        break;
      }

      head.as_mut()?.next = if l1.as_ref()?.val < l2.as_ref()?.val {
        let (origin, next) = get_next(l1);
        l1 = origin;
        next
      } else {
        let (origin, next) = get_next(l2);
        l2 = origin;
        next
      };
      head = &mut head.as_mut()?.next;
    }
    dummy_head?.next
  }
}


fn main() {
  assert_eq!(from_vec(vec![1, 2, 4]), Solution::merge_two_lists(from_vec(vec![1, 2, 4]), from_vec(vec![])));
  assert_eq!(from_vec(vec![1, 1, 2, 3, 4, 4]), Solution::merge_two_lists(from_vec(vec![1, 2, 4]), from_vec(vec![1, 3, 4])));
  assert_eq!(from_vec(vec![1, 1, 2, 3, 4, 4, 4, 5, 5, 6, 6, 7, 7]),
             Solution::merge_two_lists(from_vec(vec![1, 2, 4, 4, 5, 6, 7]),
                                       from_vec(vec![1, 3, 4, 5, 6, 7])));
  println!("Pass test cases!");
}
