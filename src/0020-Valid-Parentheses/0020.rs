/*
Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Note that an empty string is also considered valid.

Example 1:

Input: "()"
Output: true

Example 2:

Input: "()[]{}"
Output: true

Example 3:

Input: "(]"
Output: false

Example 4:

Input: "([)]"
Output: false

Example 5:

Input: "{[]}"
Output: true
*/
pub struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn is_valid(s: String) -> bool {
    let left_brackets: HashSet<char> = ['(', '[', '{'].iter().cloned().collect();
    let pair_brackets: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}')].iter().cloned().collect();

    let mut brackets_seq: VecDeque<char> = VecDeque::new();
    for ch in s.chars() {
      if left_brackets.contains(&ch) {
        brackets_seq.push_back(pair_brackets[&ch]);
      } else {
        if Some(&ch) == brackets_seq.back() {
          brackets_seq.pop_back();
        } else {
          return false;
        }
      }
    }
    return brackets_seq.is_empty();
  }
}

fn main() {
  assert_eq!(true, Solution::is_valid("()".to_string()));
  assert_eq!(true, Solution::is_valid("()[]{}".to_string()));
  assert_eq!(false, Solution::is_valid("(]".to_string()));
  assert_eq!(false, Solution::is_valid("([)]".to_string()));
  assert_eq!(true, Solution::is_valid("{[]}".to_string()));
  assert_eq!(false, Solution::is_valid("[".to_string()));
  assert_eq!(true, Solution::is_valid("".to_string()));
  println!("Pass test cases!");
}
