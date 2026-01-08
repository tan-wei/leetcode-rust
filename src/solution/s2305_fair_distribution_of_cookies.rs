/**
 * [2305] Fair Distribution of Cookies
 *
 * You are given an integer array cookies, where cookies[i] denotes the number of cookies in the i^th bag. You are also given an integer k that denotes the number of children to distribute all the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.
 * The unfairness of a distribution is defined as the maximum total cookies obtained by a single child in the distribution.
 * Return the minimum unfairness of all distributions.
 *  
 * Example 1:
 *
 * Input: cookies = [8,15,10,20,8], k = 2
 * Output: 31
 * Explanation: One optimal distribution is [8,15,8] and [10,20]
 * - The 1^st child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
 * - The 2^nd child receives [10,20] which has a total of 10 + 20 = 30 cookies.
 * The unfairness of the distribution is max(31,30) = 31.
 * It can be shown that there is no distribution with an unfairness less than 31.
 *
 * Example 2:
 *
 * Input: cookies = [6,1,3,2,2,4,1,2], k = 3
 * Output: 7
 * Explanation: One optimal distribution is [6,1], [3,2,2], and [4,1,2]
 * - The 1^st child receives [6,1] which has a total of 6 + 1 = 7 cookies.
 * - The 2^nd child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
 * - The 3^rd child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
 * The unfairness of the distribution is max(7,7,7) = 7.
 * It can be shown that there is no distribution with an unfairness less than 7.
 *
 *  
 * Constraints:
 *
 * 	2 <= cookies.length <= 8
 * 	1 <= cookies[i] <= 10^5
 * 	2 <= k <= cookies.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fair-distribution-of-cookies/
// discuss: https://leetcode.com/problems/fair-distribution-of-cookies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2305_example_1() {
        let cookies = vec![8, 15, 10, 20, 8];
        let k = 2;

        let result = 31;

        assert_eq!(Solution::distribute_cookies(cookies, k), result);
    }

    #[test]
    #[ignore]
    fn test_2305_example_2() {
        let cookies = vec![6, 1, 3, 2, 2, 4, 1, 2];
        let k = 3;

        let result = 7;

        assert_eq!(Solution::distribute_cookies(cookies, k), result);
    }
}
