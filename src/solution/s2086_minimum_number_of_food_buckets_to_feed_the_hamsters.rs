/**
 * [2086] Minimum Number of Food Buckets to Feed the Hamsters
 *
 * You are given a 0-indexed string hamsters where hamsters[i] is either:
 *
 * 	'H' indicating that there is a hamster at index i, or
 * 	'.' indicating that index i is empty.
 *
 * You will add some number of food buckets at the empty indices in order to feed the hamsters. A hamster can be fed if there is at least one food bucket to its left or to its right. More formally, a hamster at index i can be fed if you place a food bucket at index i - 1 and/or at index i + 1.
 * Return the minimum number of food buckets you should place at empty indices to feed all the hamsters or -1 if it is impossible to feed all of them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/01/example1.png" style="width: 482px; height: 162px;" />
 * Input: hamsters = "H..H"
 * Output: 2
 * Explanation: We place two food buckets at indices 1 and 2.
 * It can be shown that if we place only one food bucket, one of the hamsters will not be fed.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/01/example2.png" style="width: 602px; height: 162px;" />
 * Input: hamsters = ".H.H."
 * Output: 1
 * Explanation: We place one food bucket at index 2.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/01/example3.png" style="width: 602px; height: 162px;" />
 * Input: hamsters = ".HHH."
 * Output: -1
 * Explanation: If we place a food bucket at every empty index as shown, the hamster at index 2 will not be able to eat.
 *
 *  
 * Constraints:
 *
 * 	1 <= hamsters.length <= 10^5
 * 	hamsters[i] is either'H' or '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-food-buckets-to-feed-the-hamsters/
// discuss: https://leetcode.com/problems/minimum-number-of-food-buckets-to-feed-the-hamsters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_buckets(hamsters: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2086_example_1() {
        let hamsters = "H..H".to_string();

        let result = 2;

        assert_eq!(Solution::minimum_buckets(hamsters), result);
    }

    #[test]
    #[ignore]
    fn test_2086_example_2() {
        let hamsters = ".H.H.".to_string();

        let result = 1;

        assert_eq!(Solution::minimum_buckets(hamsters), result);
    }

    #[test]
    #[ignore]
    fn test_2086_example_3() {
        let hamsters = ".HHH.".to_string();

        let result = -1;

        assert_eq!(Solution::minimum_buckets(hamsters), result);
    }
}
