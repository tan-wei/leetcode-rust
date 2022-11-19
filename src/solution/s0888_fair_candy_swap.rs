/**
 * [0888] Fair Candy Swap
 *
 * Alice and Bob have a different total number of candies. You are given two integer arrays aliceSizes and bobSizes where aliceSizes[i] is the number of candies of the i^th box of candy that Alice has and bobSizes[j] is the number of candies of the j^th box of candy that Bob has.
 * Since they are friends, they would like to exchange one candy box each so that after the exchange, they both have the same total amount of candy. The total amount of candy a person has is the sum of the number of candies in each box they have.
 * Return an integer array answer where answer[0] is the number of candies in the box that Alice must exchange, and answer[1] is the number of candies in the box that Bob must exchange. If there are multiple answers, you may return any one of them. It is guaranteed that at least one answer exists.
 *  
 * Example 1:
 *
 * Input: aliceSizes = [1,1], bobSizes = [2,2]
 * Output: [1,2]
 *
 * Example 2:
 *
 * Input: aliceSizes = [1,2], bobSizes = [2,3]
 * Output: [1,2]
 *
 * Example 3:
 *
 * Input: aliceSizes = [2], bobSizes = [1,3]
 * Output: [2,3]
 *
 *  
 * Constraints:
 *
 * 	1 <= aliceSizes.length, bobSizes.length <= 10^4
 * 	1 <= aliceSizes[i], bobSizes[j] <= 10^5
 * 	Alice and Bob have a different total number of candies.
 * 	There will be at least one valid answer for the given input.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fair-candy-swap/
// discuss: https://leetcode.com/problems/fair-candy-swap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::iter::FromIterator;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let a_sum = alice_sizes.iter().sum::<i32>();
        let b_sum = bob_sizes.iter().sum::<i32>();
        let b_set: std::collections::HashSet<i32> =
            std::collections::HashSet::from_iter(bob_sizes.into_iter());
        for x in alice_sizes {
            let y = (b_sum - a_sum) / 2 + x;
            if b_set.contains(&y) {
                return vec![x, y];
            }
        }
        unreachable!();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0888_example_1() {
        let alice_sizes = vec![1, 1];
        let bob_sizes = vec![2, 2];
        let result = vec![1, 2];

        assert_eq!(Solution::fair_candy_swap(alice_sizes, bob_sizes), result);
    }

    #[test]
    fn test_0888_example_2() {
        let alice_sizes = vec![1, 2];
        let bob_sizes = vec![2, 3];
        let result = vec![1, 2];

        assert_eq!(Solution::fair_candy_swap(alice_sizes, bob_sizes), result);
    }

    #[test]
    fn test_0888_example_3() {
        let alice_sizes = vec![2];
        let bob_sizes = vec![1, 3];
        let result = vec![2, 3];

        assert_eq!(Solution::fair_candy_swap(alice_sizes, bob_sizes), result);
    }
}
