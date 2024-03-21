/**
 * [1528] Shuffle String
 *
 * You are given a string s and an integer array indices of the same length. The string s will be shuffled such that the character at the i^th position moves to indices[i] in the shuffled string.
 * Return the shuffled string.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/q1.jpg" style="width: 321px; height: 243px;" />
 * Input: s = "codeleet", indices = [4,5,6,7,0,2,1,3]
 * Output: "leetcode"
 * Explanation: As shown, "codeleet" becomes "leetcode" after shuffling.
 *
 * Example 2:
 *
 * Input: s = "abc", indices = [0,1,2]
 * Output: "abc"
 * Explanation: After shuffling, each character remains in its position.
 *
 *  
 * Constraints:
 *
 * 	s.length == indices.length == n
 * 	1 <= n <= 100
 * 	s consists of only lowercase English letters.
 * 	0 <= indices[i] < n
 * 	All values of indices are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shuffle-string/
// discuss: https://leetcode.com/problems/shuffle-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let pairs = indices
            .into_iter()
            .zip(s.chars())
            .collect::<std::collections::HashMap<i32, char>>();
        (0..(s.len() as i32))
            .map(|i| pairs.get(&i).unwrap())
            .collect::<String>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1528_example_1() {
        let s = "codeleet".to_string();
        let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];

        let result = "leetcode".to_string();

        assert_eq!(Solution::restore_string(s, indices), result);
    }

    #[test]
    fn test_1528_example_2() {
        let s = "abc".to_string();
        let indices = vec![0, 1, 2];

        let result = "abc".to_string();

        assert_eq!(Solution::restore_string(s, indices), result);
    }
}
