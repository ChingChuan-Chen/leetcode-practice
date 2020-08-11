/*
Determine whether an integer is a palindrome.
An integer is a palindrome when it reads the same backward as forward.

Example 1:

Input: 121
Output: true

Example 2:

Input: -121
Output: false

Explanation: From left to right, it reads -121.
From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: 10
Output: false

Explanation: Reads 01 from right to left.
Therefore it is not a palindrome.

Follow up:

Could you solve it without converting the integer to a string?

*/
pub struct Solution {}

impl Solution {
  // fastest method
  pub fn is_palindrome(x: i32) -> bool {
    let str_x = x.to_string();
    let str_x_rev = str_x.chars().rev().collect::<String>();
    return str_x == str_x_rev;
  }

  // method without converting x into string
  pub fn is_palindrome_no_string(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
      return false;
    }

    let mut input: i32 = x;
    let mut reverted_num: i32 = 0;
    while input > reverted_num {
      reverted_num = reverted_num * 10 + input % 10;
      input /= 10;
    }

    return input == reverted_num || input == reverted_num/10;
  }
}

fn main() {
  assert_eq!(true, Solution::is_palindrome(121));
  assert_eq!(true, Solution::is_palindrome(1221));
  assert_eq!(true, Solution::is_palindrome(12321));
  assert_eq!(false, Solution::is_palindrome(-121));
  assert_eq!(false, Solution::is_palindrome(10));
  // without converting to string
  assert_eq!(true, Solution::is_palindrome_no_string(121));
  assert_eq!(true, Solution::is_palindrome_no_string(1221));
  assert_eq!(true, Solution::is_palindrome_no_string(12321));
  assert_eq!(false, Solution::is_palindrome_no_string(-121));
  assert_eq!(false, Solution::is_palindrome_no_string(10));
  println!("Pass test cases!");
}
