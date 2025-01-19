/**
 * [1850] Minimum Adjacent Swaps to Reach the Kth Smallest Number
 *
 * You are given a string num, representing a large integer, and an integer k.
 * We call some integer wonderful if it is a permutation of the digits in num and is greater in value than num. There can be many wonderful integers. However, we only care about the smallest-valued ones.
 *
 * 	For example, when num = "5489355142":
 *
 * 		The 1^st smallest wonderful integer is "5489355214".
 * 		The 2^nd smallest wonderful integer is "5489355241".
 * 		The 3^rd smallest wonderful integer is "5489355412".
 * 		The 4^th smallest wonderful integer is "5489355421".
 *
 *
 *
 * Return the minimum number of adjacent digit swaps that needs to be applied to num to reach the k^th smallest wonderful integer.
 * The tests are generated in such a way that k^th smallest wonderful integer exists.
 *  
 * Example 1:
 *
 * Input: num = "5489355142", k = 4
 * Output: 2
 * Explanation: The 4^th smallest wonderful number is "5489355421". To get this number:
 * - Swap index 7 with index 8: "5489355<u>14</u>2" -> "5489355<u>41</u>2"
 * - Swap index 8 with index 9: "54893554<u>12</u>" -> "54893554<u>21</u>"
 *
 * Example 2:
 *
 * Input: num = "11112", k = 4
 * Output: 4
 * Explanation: The 4^th smallest wonderful number is "21111". To get this number:
 * - Swap index 3 with index 4: "111<u>12</u>" -> "111<u>21</u>"
 * - Swap index 2 with index 3: "11<u>12</u>1" -> "11<u>21</u>1"
 * - Swap index 1 with index 2: "1<u>12</u>11" -> "1<u>21</u>11"
 * - Swap index 0 with index 1: "<u>12</u>111" -> "<u>21</u>111"
 *
 * Example 3:
 *
 * Input: num = "00123", k = 1
 * Output: 1
 * Explanation: The 1^st smallest wonderful number is "00132". To get this number:
 * - Swap index 3 with index 4: "001<u>23</u>" -> "001<u>32</u>"
 *
 *  
 * Constraints:
 *
 * 	2 <= num.length <= 1000
 * 	1 <= k <= 1000
 * 	num only consists of digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-adjacent-swaps-to-reach-the-kth-smallest-number/
// discuss: https://leetcode.com/problems/minimum-adjacent-swaps-to-reach-the-kth-smallest-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1850_example_1() {
        let num = "5489355142".to_string();
        let k = 4;

        let result = 2;

        assert_eq!(Solution::get_min_swaps(num, k), result);
    }

    #[test]
    #[ignore]
    fn test_1850_example_2() {
        let num = "11112".to_string();
        let k = 4;

        let result = 4;

        assert_eq!(Solution::get_min_swaps(num, k), result);
    }

    #[test]
    #[ignore]
    fn test_1850_example_3() {
        let num = "00123".to_string();
        let k = 1;

        let result = 1;

        assert_eq!(Solution::get_min_swaps(num, k), result);
    }
}
