/**
 * [1718] Construct the Lexicographically Largest Valid Sequence
 *
 * Given an integer n, find a sequence that satisfies all of the following:
 *
 * 	The integer 1 occurs once in the sequence.
 * 	Each integer between 2 and n occurs twice in the sequence.
 * 	For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.
 *
 * The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their indices, |j - i|.
 * Return the lexicographically largest sequence. It is guaranteed that under the given constraints, there is always a solution.
 * A sequence a is lexicographically larger than a sequence b (of the same length) if in the first position where a and b differ, sequence a has a number greater than the corresponding number in b. For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they differ is at the third number, and 9 is greater than 5.
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: [3,1,2,3,2]
 * Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
 *
 * Example 2:
 *
 * Input: n = 5
 * Output: [5,3,1,4,3,5,2,4,2]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/
// discuss: https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result = vec![0; 2 * n - 1];
        let mut used = vec![false; n + 1];

        Self::dfs_helper(0, &mut result, &mut used);

        result
    }

    fn dfs_helper(pos: usize, result: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
        if pos == result.len() {
            return true;
        }

        if result[pos] != 0 {
            return Self::dfs_helper(pos + 1, result, used);
        }

        for i in (1..used.len()).rev() {
            if used[i] {
                continue;
            }
            if i != 1 && (pos + i >= result.len() || result[pos + i] != 0) {
                continue;
            }
            result[pos] = i as i32;
            if i != 1 {
                result[pos + i] = i as i32;
            }
            used[i] = true;
            if Self::dfs_helper(pos + 1, result, used) {
                return true;
            }
            result[pos] = 0;
            if i != 1 {
                result[pos + i] = 0;
            }
            used[i] = false;
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1718_example_1() {
        let n = 3;

        let result = vec![3, 1, 2, 3, 2];

        assert_eq!(Solution::construct_distanced_sequence(n), result);
    }

    #[test]
    #[ignore]
    fn test_1718_example_2() {
        let n = 5;

        let result = vec![5, 3, 1, 4, 3, 5, 2, 4, 2];

        assert_eq!(Solution::construct_distanced_sequence(n), result);
    }
}
