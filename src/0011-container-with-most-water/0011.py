# Given n non-negative integers a1, a2, ..., an , where each represents a point at
# coordinate (i, ai). n vertical lines are drawn such that the two endpoints of line i is
# at (i, ai) and (i, 0). Find two lines, which together with x-axis forms a container,
# such that the container contains the most water.
#
#  Note: You may not slant the container and n is at least 2.
#
#  The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case,
#  the max area of water (blue section) the container can contain is 49.
#
#
#
# Example:
# Input: [1,8,6,2,5,4,8,3,7]
# Output: 49 Related Topics Array Two Pointers

from typing import List
class Solution:
    def maxArea(self, height: List[int]) -> int:
        n = len(height)
        max_area = 0
        l = 0
        r = n-1
        while l <= r:
            max_area = max(max_area, min(height[l], height[r]) * (r-l))
            if height[l] > height[r]:
                r -= 1
            else:
                l += 1
        return max_area

if __name__ == '__main__':
    assert Solution().maxArea([1,8,6,2,5,4,8,3,7]) == 49
    assert Solution().maxArea([1, 1]) == 1
    assert Solution().maxArea([1, 3]) == 1
    assert Solution().maxArea([4, 3]) == 3
    assert Solution().maxArea([5, 6, 4, 3, 2]) == 9
