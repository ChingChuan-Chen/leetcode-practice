/*
Given an integer array nums, find the contiguous subarray (containing at least one number)

which has the largest sum and return its sum.

Example:

Input: [-2,1,-3,4,-1,2,1,-5,4],
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.

Follow up:
If you have figured out the O(n) solution, try coding another solution using
the divide and conquer approach, which is more subtle.
*/
pub struct Solution {}

// reference: https://emn178.pixnet.net/blog/post/88907691

// O(n) solution
/*
impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }

    let mut s = 0;
    let mut max = i32::min_value();
    for i in 0..nums.len() {
      s += nums[i];
      max = i32::max(max, s);
      if s <= 0 {
        s = 0;
      }
    }
    max
  }
}
*/

// divide and conquer solution
impl Solution {
  pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }

    let mut s = 0;
    let mut max = i32::min_value();
    for i in 0..nums.len() {
      s += nums[i];
      max = i32::max(max, s);
      if s <= 0 {
        s = 0;
      }
    }
    max
  }
}

fn main() {
  assert_eq!(6, Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
  assert_eq!(-8, Solution::max_sub_array(vec![-8]));
  assert_eq!(-2, Solution::max_sub_array(vec![-2, -8]));
  println!("Pass test cases!");
}
