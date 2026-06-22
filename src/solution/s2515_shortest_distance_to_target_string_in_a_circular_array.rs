/**
 * [2515] Shortest Distance to Target String in a Circular Array
 *
 * You are given a 0-indexed circular string array words and a string target. A circular array means that the array's end connects to the array's beginning.
 *
 * 	Formally, the next element of words[i] is words[(i + 1) % n] and the previous element of words[i] is words[(i - 1 + n) % n], where n is the length of words.
 *
 * Starting from startIndex, you can move to either the next word or the previous word with 1 step at a time.
 * Return the shortest distance needed to reach the string target. If the string target does not exist in words, return -1.
 *  
 * Example 1:
 *
 * Input: words = ["hello","i","am","leetcode","hello"], target = "hello", startIndex = 1
 * Output: 1
 * Explanation: We start from index 1 and can reach "hello" by
 * - moving 3 units to the right to reach index 4.
 * - moving 2 units to the left to reach index 4.
 * - moving 4 units to the right to reach index 0.
 * - moving 1 unit to the left to reach index 0.
 * The shortest distance to reach "hello" is 1.
 *
 * Example 2:
 *
 * Input: words = ["a","b","leetcode"], target = "leetcode", startIndex = 0
 * Output: 1
 * Explanation: We start from index 0 and can reach "leetcode" by
 * - moving 2 units to the right to reach index 2.
 * - moving 1 unit to the left to reach index 2.
 * The shortest distance to reach "leetcode" is 1.
 * Example 3:
 *
 * Input: words = ["i","eat","leetcode"], target = "ate", startIndex = 0
 * Output: -1
 * Explanation: Since "ate" does not exist in words, we return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 100
 * 	words[i] and target consist of only lowercase English letters.
 * 	0 <= startIndex < words.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/
// discuss: https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closest_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let n = words.len() as i32;
        (0..n)
            .filter_map(|i| {
                if words[((start_index + i) % n) as usize] == target
                    || words[(((start_index - i) + n) % n) as usize] == target
                {
                    Some(i)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2515_example_1() {
        let words = vec_string!["hello", "i", "am", "leetcode", "hello"];
        let target = "hello".to_string();
        let start_index = 1;

        let result = 1;

        assert_eq!(Solution::closest_target(words, target, start_index), result);
    }

    #[test]
    fn test_2515_example_2() {
        let words = vec_string!["a", "b", "leetcode"];
        let target = "leetcode".to_string();
        let start_index = 0;

        let result = 1;

        assert_eq!(Solution::closest_target(words, target, start_index), result);
    }

    #[test]
    fn test_2515_example_3() {
        let words = vec_string!["i", "eat", "leetcode"];
        let target = "ate".to_string();
        let start_index = 0;

        let result = -1;

        assert_eq!(Solution::closest_target(words, target, start_index), result);
    }
}
