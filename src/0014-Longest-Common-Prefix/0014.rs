/*
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Note:
All given inputs are in lowercase letters a-z.
*/
pub struct Solution {}

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() < 1 {
      return String::new();
    }

    let mut prefix = strs.get(0).unwrap().to_string();
    for i in 1..strs.len() {
      let mut match_char: Vec<char> = Vec::new();
      for (c1, c2) in prefix.chars().zip(strs[i].chars()) {
        if c1 == c2 {
          match_char.push(c1);
        } else {
          break;
        }
      }
      prefix = match_char.iter().collect();
      if prefix.len() == 0 {
        return String::new();
      }
    }
    return prefix;
  }
}

fn main() {
  assert_eq!("", Solution::longest_common_prefix(vec![]));
  assert_eq!("fl", Solution::longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]));
  assert_eq!("", Solution::longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]));
  assert_eq!("", Solution::longest_common_prefix(vec!["aca".to_string(),"cba".to_string()]));
  println!("Pass test cases!");
}
