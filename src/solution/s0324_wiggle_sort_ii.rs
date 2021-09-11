/**
 * [324] Wiggle Sort II
 *
 * Given an integer array nums, reorder it such that nums[0] < nums[1] > nums[2] < nums[3]....
 * You may assume the input array always has a valid answer.
 *  
 * Example 1:
 *
 * Input: nums = [1,5,1,1,6,4]
 * Output: [1,6,1,5,1,4]
 * Explanation: [1,4,1,5,1,6] is also accepted.
 *
 * Example 2:
 *
 * Input: nums = [1,3,2,2,3,1]
 * Output: [2,3,1,3,1,2]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	0 <= nums[i] <= 5000
 * 	It is guaranteed that there will be an answer for the given input nums.
 *
 *  
 * Follow Up: Can you do it in O(n) time and/or in-place with O(1) extra space?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/wiggle-sort-ii/
// discuss: https://leetcode.com/problems/wiggle-sort-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        fn get_middle(v: &mut [i32], l: usize, r: usize) -> usize {
            let mut i = l;
            for j in l + 1..=r {
                if v[j] < v[l] {
                    i += 1;
                    v.swap(i, j)
                }
            }
            v.swap(l, i);
            i
        }

        fn kth_smallest(nums: &mut Vec<i32>, mut k: usize) -> i32 {
            let mut l = 0;
            let mut r = nums.len() - 1;
            k -= 1;
            while l < r {
                let m = get_middle(nums, l, r);
                if m < k {
                    l = m + 1;
                } else if m > k {
                    r = m - 1;
                } else {
                    break;
                }
            }
            nums[k]
        }

        let n = nums.len();
        let m = (n + 1) / 2;
        let median = kth_smallest(nums, m);

        let mut i = 0;
        let mut k = n - 1;
        let mut j = 0;
        while j <= k {
            if nums[(2 * j + 1) % (n | 1)] > median {
                nums.swap((2 * i + 1) % (n | 1), (2 * j + 1) % (n | 1));
                i += 1;
                j += 1;
            } else if nums[(2 * j + 1) % (n | 1)] < median {
                nums.swap((2 * j + 1) % (n | 1), (2 * k + 1) % (n | 1));
                k -= 1;
            } else {
                j += 1;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0324_example_1() {
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        let result = vec![1, 6, 1, 5, 1, 4];

        Solution::wiggle_sort(&mut nums);

        assert_eq!(nums, result);
    }

    #[test]
    #[ignore]
    fn test_0324_example_2() {
        let mut nums = vec![1, 3, 2, 2, 3, 1];
        let result = vec![2, 3, 1, 3, 1, 2];

        Solution::wiggle_sort(&mut nums);

        assert_eq!(nums, result);
    }
}
