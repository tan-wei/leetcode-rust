/**
 * [2222] Number of Ways to Select Buildings
 *
 * You are given a 0-indexed binary string s which represents the types of buildings along a street where:
 *
 * 	s[i] = '0' denotes that the i^th building is an office and
 * 	s[i] = '1' denotes that the i^th building is a restaurant.
 *
 * As a city official, you would like to select 3 buildings for random inspection. However, to ensure variety, no two consecutive buildings out of the selected buildings can be of the same type.
 *
 * 	For example, given s = "0<u>0</u>1<u>1</u>0<u>1</u>", we cannot select the 1^st, 3^rd, and 5^th buildings as that would form "0<u>11</u>" which is not allowed due to having two consecutive buildings of the same type.
 *
 * Return the number of valid ways to select 3 buildings.
 *  
 * Example 1:
 *
 * Input: s = "001101"
 * Output: 6
 * Explanation:
 * The following sets of indices selected are valid:
 * - [0,2,4] from "<u>0</u>0<u>1</u>1<u>0</u>1" forms "010"
 * - [0,3,4] from "<u>0</u>01<u>10</u>1" forms "010"
 * - [1,2,4] from "0<u>01</u>1<u>0</u>1" forms "010"
 * - [1,3,4] from "0<u>0</u>1<u>10</u>1" forms "010"
 * - [2,4,5] from "00<u>1</u>1<u>01</u>" forms "101"
 * - [3,4,5] from "001<u>101</u>" forms "101"
 * No other selection is valid. Thus, there are 6 total ways.
 *
 * Example 2:
 *
 * Input: s = "11100"
 * Output: 0
 * Explanation: It can be shown that there are no valid selections.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-select-buildings/
// discuss: https://leetcode.com/problems/number-of-ways-to-select-buildings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_ways(s: String) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2222_example_1() {
        let s = "001101".to_string();

        let result = 6;

        assert_eq!(Solution::number_of_ways(s), result);
    }

    #[test]
    #[ignore]
    fn test_2222_example_2() {
        let s = "11100".to_string();

        let result = 0;

        assert_eq!(Solution::number_of_ways(s), result);
    }
}
