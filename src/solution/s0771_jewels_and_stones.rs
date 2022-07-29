/**
 * [0771] Jewels and Stones
 *
 * You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.
 * Letters are case sensitive, so "a" is considered a different type of stone from "A".
 *  
 * Example 1:
 * Input: jewels = "aA", stones = "aAAbbbb"
 * Output: 3
 * Example 2:
 * Input: jewels = "z", stones = "ZZ"
 * Output: 0
 *  
 * Constraints:
 *
 * 	1 <= jewels.length, stones.length <= 50
 * 	jewels and stones consist of only English letters.
 * 	All the characters of jewels are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jewels-and-stones/
// discuss: https://leetcode.com/problems/jewels-and-stones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut set = jewels.chars().collect::<std::collections::HashSet<char>>();
        stones
            .chars()
            .fold(0, |acc, ch| if set.contains(&ch) { acc + 1 } else { acc })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0771_example_1() {
        let jewels = "aA".to_string();
        let stones = "aAAbbbb".to_string();
        let result = 3;

        assert_eq!(Solution::num_jewels_in_stones(jewels, stones), result);
    }

    #[test]
    fn test_0771_example_2() {
        let jewels = "z".to_string();
        let stones = "ZZ".to_string();
        let result = 0;

        assert_eq!(Solution::num_jewels_in_stones(jewels, stones), result);
    }
}
