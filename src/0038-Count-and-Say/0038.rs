/*
The count-and-say sequence is the sequence of integers with the first five terms as following:

1.     1
2.     11
3.     21
4.     1211
5.     111221

1 is read off as "one 1" or 11.
11 is read off as "two 1s" or 21.
21 is read off as "one 2, then one 1" or 1211.

Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say sequence.

Note: Each term of the sequence of integers will be represented as a string.

Example 1:

Input: 1
Output: "1"

Example 2:

Input: 4
Output: "1211"
*/
pub struct Solution {}

use std::char::from_digit;
impl Solution {
  pub fn count_and_say(n: i32) -> String {
    let mut res: Vec<char> = vec!['1'];
    for _i in 1..n {
      let mut temp: Vec<char> = Vec::new();
      let mut j = 0;
      let mut num = 0;
      for (pos, c) in res.iter().enumerate() {
        if *c == res[j] {
          num += 1;
        } else {
          temp.push(from_digit(num, 10).unwrap());
          temp.push(res[j]);
          num = 1;
          j = pos;
        }
      }
      temp.push(from_digit(num, 10).unwrap());
      temp.push(res[j]);
      res = temp;
    }
    return res.iter().collect();
  }
}

fn main() {
  assert_eq!("1".to_string(), Solution::count_and_say(1));
  assert_eq!("11".to_string(), Solution::count_and_say(2));
  assert_eq!("21".to_string(), Solution::count_and_say(3));
  assert_eq!("1211".to_string(), Solution::count_and_say(4));
  assert_eq!("111221".to_string(), Solution::count_and_say(5));
  assert_eq!("312211".to_string(), Solution::count_and_say(6));
  println!("Pass test cases!");
}
