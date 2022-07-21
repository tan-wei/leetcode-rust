/**
 * [0763] Partition Labels
 *
 * You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
 * Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.
 * Return a list of integers representing the size of these parts.
 *  
 * Example 1:
 *
 * Input: s = "ababcbacadefegdehijhklij"
 * Output: [9,7,8]
 * Explanation:
 * The partition is "ababcbaca", "defegde", "hijhklij".
 * This is a partition so that each letter appears in at most one part.
 * A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
 *
 * Example 2:
 *
 * Input: s = "eccbbbbdec"
 * Output: [10]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-labels/
// discuss: https://leetcode.com/problems/partition-labels/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_pos = std::collections::HashMap::new();

        for (i, c) in s.bytes().enumerate() {
            last_pos.insert(c, i);
        }

        let mut result = vec![];
        let mut cur_last_pos = i32::MIN;
        let mut last_i = 0;
        for (i, c) in s.bytes().enumerate() {
            cur_last_pos = std::cmp::max(cur_last_pos, *last_pos.get(&c).unwrap() as i32);
            if cur_last_pos == i as i32 {
                result.push((i - last_i + 1) as i32);
                last_i = i + 1;
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0763_example_1() {
        let s = "ababcbacadefegdehijhklij".to_string();
        let result = vec![9, 7, 8];

        assert_eq!(Solution::partition_labels(s), result);
    }

    #[test]
    fn test_0763_example_2() {
        let s = "eccbbbbdec".to_string();
        let result = vec![10];

        assert_eq!(Solution::partition_labels(s), result);
    }
}
