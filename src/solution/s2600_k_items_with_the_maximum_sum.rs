/**
 * [2600] K Items With the Maximum Sum
 *
 * There is a bag that consists of items, each item has a number 1, 0, or -1 written on it.
 * You are given four non-negative integers numOnes, numZeros, numNegOnes, and k.
 * The bag initially contains:
 *
 * 	numOnes items with 1s written on them.
 * 	numZeroes items with 0s written on them.
 * 	numNegOnes items with -1s written on them.
 *
 * We want to pick exactly k items among the available items. Return the maximum possible sum of numbers written on the items.
 *  
 * Example 1:
 *
 * Input: numOnes = 3, numZeros = 2, numNegOnes = 0, k = 2
 * Output: 2
 * Explanation: We have a bag of items with numbers written on them {1, 1, 1, 0, 0}. We take 2 items with 1 written on them and get a sum in a total of 2.
 * It can be proven that 2 is the maximum possible sum.
 *
 * Example 2:
 *
 * Input: numOnes = 3, numZeros = 2, numNegOnes = 0, k = 4
 * Output: 3
 * Explanation: We have a bag of items with numbers written on them {1, 1, 1, 0, 0}. We take 3 items with 1 written on them, and 1 item with 0 written on it, and get a sum in a total of 3.
 * It can be proven that 3 is the maximum possible sum.
 *
 *  
 * Constraints:
 *
 * 	0 <= numOnes, numZeros, numNegOnes <= 50
 * 	0 <= k <= numOnes + numZeros + numNegOnes
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-items-with-the-maximum-sum/
// discuss: https://leetcode.com/problems/k-items-with-the-maximum-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        num_ones.min(k) - (k - num_ones - num_zeros).max(0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2600_example_1() {
        let num_ones = 3;
        let num_zeros = 2;
        let num_neg_ones = 0;
        let k = 2;

        let result = 2;

        assert_eq!(
            Solution::k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k),
            result
        );
    }

    #[test]
    fn test_2600_example_2() {
        let num_ones = 3;
        let num_zeros = 2;
        let num_neg_ones = 0;
        let k = 4;

        let result = 3;

        assert_eq!(
            Solution::k_items_with_maximum_sum(num_ones, num_zeros, num_neg_ones, k),
            result
        );
    }
}
