/*
Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "hello", needle = "ll"
Output: 2

Example 2:

Input: haystack = "aaaaa", needle = "bba"
Output: -1

Clarification:

What should we return when needle is an empty string? This is a great question to ask during an interview.

For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().
*/
pub struct Solution {}

impl Solution {
  pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
      return 0;
    }
    if haystack.len() < needle.len() {
      return -1;
    }
    // easy solution
    // haystack.find(&needle).map_or(-1_i32, |v| v as i32)

    // naive solution
    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let mut curr_idx = 0;
    let stop_pos = haystack_chars.len() - needle_chars.len() + 1;
    loop {
      let mut i = curr_idx;
      let mut j = 0;
      while haystack_chars[i] == needle_chars[j] {
        i += 1;
        j += 1;
        if j == needle_chars.len() {
          return curr_idx as i32;
        }
      }
      curr_idx += 1;
      if curr_idx >= stop_pos {
        break;
      }
    }
    return -1;
  }
}

fn main() {
  assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
  assert_eq!(3, Solution::str_str("hello".to_string(), "lo".to_string()));
  assert_eq!(0, Solution::str_str("hello".to_string(), "hello".to_string()));
  assert_eq!(-1, Solution::str_str("aaaaa".to_string(), "baa".to_string()));
  assert_eq!(0, Solution::str_str("aaaaa".to_string(), "".to_string()));
  assert_eq!(-1, Solution::str_str("".to_string(), "baa".to_string()));
  println!("Pass test cases!");
}
