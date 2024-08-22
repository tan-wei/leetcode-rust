/**
 * [1655] Distribute Repeating Integers
 *
 * You are given an array of n integers, nums, where there are at most 50 unique values in the array. You are also given an array of m customer order quantities, quantity, where quantity[i] is the amount of integers the i^th customer ordered. Determine if it is possible to distribute nums such that:
 *
 * 	The i^th customer gets exactly quantity[i] integers,
 * 	The integers the i^th customer gets are all equal, and
 * 	Every customer is satisfied.
 *
 * Return true if it is possible to distribute nums according to the above conditions.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4], quantity = [2]
 * Output: false
 * Explanation: The 0^th customer cannot be given two different integers.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,3], quantity = [2]
 * Output: true
 * Explanation: The 0^th customer is given [3,3]. The integers [1,2] are not used.
 *
 * Example 3:
 *
 * Input: nums = [1,1,2,2], quantity = [2,2]
 * Output: true
 * Explanation: The 0^th customer is given [1,1], and the 1st customer is given [2,2].
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	1 <= nums[i] <= 1000
 * 	m == quantity.length
 * 	1 <= m <= 10
 * 	1 <= quantity[i] <= 10^5
 * 	There are at most 50 unique values in nums.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distribute-repeating-integers/
// discuss: https://leetcode.com/problems/distribute-repeating-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/distribute-repeating-integers/solutions/3187911/just-a-runnable-solution/
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut counts = std::collections::HashMap::new();
        for n in nums.iter() {
            *counts.entry(*n).or_insert(0) += 1;
        }
        let mut counts = counts.values().copied().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        let mut quantity = quantity;
        quantity.sort_by(|a, b| b.cmp(a));

        Self::dfs_helper(&mut counts, &quantity, 0)
    }

    fn dfs_helper(counts: &mut Vec<i32>, quantity: &Vec<i32>, index: usize) -> bool {
        if index == quantity.len() {
            return true;
        }
        for i in 0..counts.len() {
            if counts[i] >= quantity[index] {
                let p = quantity[index];
                counts[i] -= p;
                if Self::dfs_helper(counts, quantity, index + 1) {
                    return true;
                }
                counts[i] += p;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1655_example_1() {
        let nums = vec![1, 2, 3, 4];
        let quantity = vec![2];

        let result = false;

        assert_eq!(Solution::can_distribute(nums, quantity), result);
    }

    #[test]
    fn test_1655_example_2() {
        let nums = vec![1, 2, 3, 3];
        let quantity = vec![2];

        let result = true;

        assert_eq!(Solution::can_distribute(nums, quantity), result);
    }

    #[test]
    fn test_1655_example_3() {
        let nums = vec![1, 1, 2, 2];
        let quantity = vec![2, 2];

        let result = true;

        assert_eq!(Solution::can_distribute(nums, quantity), result);
    }
}
