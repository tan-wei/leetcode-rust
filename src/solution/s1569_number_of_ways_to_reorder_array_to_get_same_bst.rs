/**
 * [1569] Number of Ways to Reorder Array to Get Same BST
 *
 * Given an array nums that represents a permutation of integers from 1 to n. We are going to construct a binary search tree (BST) by inserting the elements of nums in order into an initially empty BST. Find the number of different ways to reorder nums so that the constructed BST is identical to that formed from the original array nums.
 *
 * 	For example, given nums = [2,1,3], we will have 2 as the root, 1 as a left child, and 3 as a right child. The array [2,3,1] also yields the same BST but [3,2,1] yields a different BST.
 *
 * Return the number of ways to reorder nums such that the BST formed is identical to the original BST formed from nums.
 * Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/bb.png" style="width: 121px; height: 101px;" />
 * Input: nums = [2,1,3]
 * Output: 1
 * Explanation: We can reorder nums to be [2,3,1] which will yield the same BST. There are no other ways to reorder nums which will yield the same BST.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/ex1.png" style="width: 241px; height: 161px;" />
 * Input: nums = [3,4,5,1,2]
 * Output: 5
 * Explanation: The following 5 arrays will yield the same BST:
 * [3,1,2,4,5]
 * [3,1,4,2,5]
 * [3,1,4,5,2]
 * [3,4,1,2,5]
 * [3,4,1,5,2]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/12/ex4.png" style="width: 121px; height: 161px;" />
 * Input: nums = [1,2,3]
 * Output: 0
 * Explanation: There are no other orderings of nums that will yield the same BST.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= nums.length
 * 	All integers in nums are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/
// discuss: https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/solutions/3643480/rust-dfs-solution/

    const MOD: i32 = 1_000_000_007;

    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut cnk = vec![vec![1; n + 1]; n + 1];

        for i in 1..n + 1 {
            for j in 1..i {
                cnk[i][j] = (cnk[i - 1][j] + cnk[i - 1][j - 1]) % Self::MOD;
            }
        }

        Self::dfs_helper(nums, &cnk) - 1
    }

    fn dfs_helper(nums: Vec<i32>, cnk: &Vec<Vec<i32>>) -> i32 {
        let n = nums.len();

        if n <= 2 {
            return 1;
        }

        let (mut left, mut right) = (vec![], vec![]);

        for i in 1..n {
            if nums[i] < nums[0] {
                left.push(nums[i]);
            } else {
                right.push(nums[i]);
            }
        }

        let mut result = cnk[n - 1][left.len()] as i64;
        result = (result * Self::dfs_helper(left, cnk) as i64) % Self::MOD as i64;
        result = (result * Self::dfs_helper(right, cnk) as i64) % Self::MOD as i64;

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1569_example_1() {
        let nums = vec![2, 1, 3];

        let result = 1;

        assert_eq!(Solution::num_of_ways(nums), result);
    }

    #[test]
    fn test_1569_example_2() {
        let nums = vec![3, 4, 5, 1, 2];

        let result = 5;

        assert_eq!(Solution::num_of_ways(nums), result);
    }

    #[test]
    fn test_1569_example_3() {
        let nums = vec![1, 2, 3];

        let result = 0;

        assert_eq!(Solution::num_of_ways(nums), result);
    }
}
