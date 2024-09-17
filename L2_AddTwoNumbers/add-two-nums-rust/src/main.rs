// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
// 
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {

}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // A fascinating thing about Rust is the way it manages memory for efficient use.
        // Option is a data type for values that may not exist, i.e., None.
        // Null pointers (pointers that point to no value) are dangerous as it can lead to crashes or data corruption.
        // Thus, Rust compels us to handle cases where value could be None, thus avoiding null pointer errors.
        // Box is a smart pointer. It's smart because while it simply allocates data from heap,
        // it automatically manages memory, i.e., it drops content when going out of scope.
        let mut carry = 0;
        // This will be useful to store the carryover digit during addition
        let mut dummy_head = Box::new(ListNode::new(0)); // This will be the starting point of linked list
        let mut current = &mut dummy_head;

        let mut l1 = l1;
        let mut l2 = l2;
        // both l1 and l2 are mutable now
        // is_some() is used to check if an object has some value
        // OR is null (which will break the loop condition)
        while l1.is_some() || l2.is_some() || carry != 0 {
            // in below two statements, 0 is a default value and |node| is similar to `lambda x` in python
            // This is Option but in ternary operator format, if node is not null then we access value else we return 0
            let val1 = l1.as_ref().map_or(0, |node| node.val);
            let val2 = l2.as_ref().map_or(0, |node| node.val);
            // As we iterate over digits, we take their sum (and carryover as well)
            let sum = val1 + val2 + carry;
            carry = sum / 10;
            // A new node is now created to store the single digit sum
            // the carryover will be useful in next iteration (especially when there are no digits left)
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            // both as_ref and as_mut provide reference to the value inside the Option (if it exists)
            // and avoid unneccessary cloning (but still return an Option)
            // as_mut however can modify the value inside Option
            // unwrap is used to extract value from an option 
            // For example, if we have `let x = Some(5);`, and want to assign x's value to a variable y.
            // It can be done through `let y = x.unwrap();`
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
            // and_then, like map_or, is used to safely handle Option values.
            // We can chain operations and safely avoid null pointer cases.
        }

        dummy_head.next // return the result
    }
}

#[test]
fn test_simple_sum() {
  let l1 = Some(Box::new(ListNode::new(9)));
  let l2 = Some(Box::new(ListNode::new(0)));
  let result = Solution::add_two_numbers(l1, l2);
  let answer = Some(Box::new(ListNode { val: 9, next: None}));
  assert!(result == answer);
}

#[test]
fn test_three_digit() {
  let mut l1 = Box::new(ListNode::new(6));
  l1.next = Some(Box::new(ListNode::new(9)));
  let mut l2 = Box::new(ListNode::new(9));
  l2.next = Some(Box::new(ListNode::new(6)));
  let result = Solution::add_two_numbers(Some(l1), Some(l2));
  let answer = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 1, next: None })) })) }));
  assert_eq!(result, answer);
}

#[test]
fn test_not_equal() {
  let mut l1 = Box::new(ListNode::new(0));
  let mut l2 = Box::new(ListNode::new(0));
  let result = Solution::add_two_numbers(Some(l1), Some(l2));
  let answer = Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 1, next: None })) }));
  assert_ne!(result, answer);
}

fn main() {
  println!("Solution and Test for Add Two Nums");
}