/**
 * [2155] All Divisions With the Highest Score of a Binary Array
 *
 * You are given a 0-indexed binary array nums of length n. nums can be divided at index i (where 0 <= i <= n) into two arrays (possibly empty) numsleft and numsright:
 *
 * 	numsleft has all the elements of nums between index 0 and i - 1 (inclusive), while numsright has all the elements of nums between index i and n - 1 (inclusive).
 * 	If i == 0, numsleft is empty, while numsright has all the elements of nums.
 * 	If i == n, numsleft has all the elements of nums, while numsright is empty.
 *
 * The division score of an index i is the sum of the number of 0's in numsleft and the number of 1's in numsright.
 * Return all distinct indices that have the highest possible division score. You may return the answer in any order.
 *  
 * Example 1:
 *
 * Input: nums = [0,0,1,0]
 * Output: [2,4]
 * Explanation: Division at index
 * - 0: numsleft is []. numsright is [0,0,<u>1</u>,0]. The score is 0 + 1 = 1.
 * - 1: numsleft is [<u>0</u>]. numsright is [0,<u>1</u>,0]. The score is 1 + 1 = 2.
 * - 2: numsleft is [<u>0</u>,<u>0</u>]. numsright is [<u>1</u>,0]. The score is 2 + 1 = 3.
 * - 3: numsleft is [<u>0</u>,<u>0</u>,1]. numsright is [0]. The score is 2 + 0 = 2.
 * - 4: numsleft is [<u>0</u>,<u>0</u>,1,<u>0</u>]. numsright is []. The score is 3 + 0 = 3.
 * Indices 2 and 4 both have the highest possible division score 3.
 * Note the answer [4,2] would also be accepted.
 * Example 2:
 *
 * Input: nums = [0,0,0]
 * Output: [3]
 * Explanation: Division at index
 * - 0: numsleft is []. numsright is [0,0,0]. The score is 0 + 0 = 0.
 * - 1: numsleft is [<u>0</u>]. numsright is [0,0]. The score is 1 + 0 = 1.
 * - 2: numsleft is [<u>0</u>,<u>0</u>]. numsright is [0]. The score is 2 + 0 = 2.
 * - 3: numsleft is [<u>0</u>,<u>0</u>,<u>0</u>]. numsright is []. The score is 3 + 0 = 3.
 * Only index 3 has the highest possible division score 3.
 *
 * Example 3:
 *
 * Input: nums = [1,1]
 * Output: [0]
 * Explanation: Division at index
 * - 0: numsleft is []. numsright is [<u>1</u>,<u>1</u>]. The score is 0 + 2 = 2.
 * - 1: numsleft is [1]. numsright is [<u>1</u>]. The score is 0 + 1 = 1.
 * - 2: numsleft is [1,1]. numsright is []. The score is 0 + 0 = 0.
 * Only index 0 has the highest possible division score 2.
 *
 *  
 * Constraints:
 *
 * 	n == nums.length
 * 	1 <= n <= 10^5
 * 	nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/all-divisions-with-the-highest-score-of-a-binary-array/
// discuss: https://leetcode.com/problems/all-divisions-with-the-highest-score-of-a-binary-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2155_example_1() {
        let nums = vec![0, 0, 1, 0];

        let result = vec![2, 4];

        assert_eq!(Solution::max_score_indices(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2155_example_2() {
        let nums = vec![0, 0, 0];

        let result = vec![3];

        assert_eq!(Solution::max_score_indices(nums), result);
    }

    #[test]
    #[ignore]
    fn test_2155_example_3() {
        let nums = vec![1, 1];

        let result = vec![0];

        assert_eq!(Solution::max_score_indices(nums), result);
    }
}
