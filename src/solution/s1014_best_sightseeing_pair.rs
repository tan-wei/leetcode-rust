/**
 * [1014] Best Sightseeing Pair
 *
 * You are given an integer array values where values[i] represents the value of the i^th sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.
 * The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.
 * Return the maximum score of a pair of sightseeing spots.
 *  
 * Example 1:
 *
 * Input: values = [8,1,5,2,6]
 * Output: 11
 * Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
 *
 * Example 2:
 *
 * Input: values = [1,2]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	2 <= values.length <= 5 * 10^4
 * 	1 <= values[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-sightseeing-pair/
// discuss: https://leetcode.com/problems/best-sightseeing-pair/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut result = 0;

        for value in values {
            result = std::cmp::max(result, cur + value);
            cur = std::cmp::max(cur, value) - 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1014_example_1() {
        let values = vec![8, 1, 5, 2, 6];
        let result = 11;

        assert_eq!(Solution::max_score_sightseeing_pair(values), result);
    }

    #[test]
    fn test_1014_example_2() {
        let values = vec![1, 2];
        let result = 2;

        assert_eq!(Solution::max_score_sightseeing_pair(values), result);
    }
}
