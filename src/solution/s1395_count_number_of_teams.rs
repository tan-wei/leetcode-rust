/**
 * [1395] Count Number of Teams
 *
 * There are n soldiers standing in a line. Each soldier is assigned a unique rating value.
 * You have to form a team of 3 soldiers amongst them under the following rules:
 *
 * 	Choose 3 soldiers with index (i, j, k) with rating (rating[i], rating[j], rating[k]).
 * 	A team is valid if: (rating[i] < rating[j] < rating[k]) or (rating[i] > rating[j] > rating[k]) where (0 <= i < j < k < n).
 *
 * Return the number of teams you can form given the conditions. (soldiers can be part of multiple teams).
 *  
 * Example 1:
 *
 * Input: rating = [2,5,3,4,1]
 * Output: 3
 * Explanation: We can form three teams given the conditions. (2,3,4), (5,4,1), (5,3,1).
 *
 * Example 2:
 *
 * Input: rating = [2,1,3]
 * Output: 0
 * Explanation: We can't form any team given the conditions.
 *
 * Example 3:
 *
 * Input: rating = [1,2,3,4]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	n == rating.length
 * 	3 <= n <= 1000
 * 	1 <= rating[i] <= 10^5
 * 	All the integers in rating are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-number-of-teams/
// discuss: https://leetcode.com/problems/count-number-of-teams/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let arg = rating.len();
        let mut inc = vec![0; arg];
        let mut dec = vec![0; arg];
        let mut num_teams = 0;

        for i in 0..arg {
            for j in (i + 1)..arg {
                if rating[j] > rating[i] {
                    inc[j] += 1;
                    num_teams += inc[i];
                } else if rating[j] < rating[i] {
                    dec[j] += 1;
                    num_teams += dec[i];
                }
            }
        }

        num_teams
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1395_example_1() {
        let rating = vec![2, 5, 3, 4, 1];

        let result = 3;

        assert_eq!(Solution::num_teams(rating), result);
    }

    #[test]
    fn test_1395_example_2() {
        let rating = vec![2, 1, 3];

        let result = 0;

        assert_eq!(Solution::num_teams(rating), result);
    }

    #[test]
    fn test_1395_example_3() {
        let rating = vec![1, 2, 3, 4];

        let result = 4;

        assert_eq!(Solution::num_teams(rating), result);
    }
}
