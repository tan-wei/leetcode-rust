/**
 * [0899] Orderly Queue
 *
 * You are given a string s and an integer k. You can choose one of the first k letters of s and append it at the end of the string..
 * Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.
 *  
 * Example 1:
 *
 * Input: s = "cba", k = 1
 * Output: "acb"
 * Explanation:
 * In the first move, we move the 1^st character 'c' to the end, obtaining the string "bac".
 * In the second move, we move the 1^st character 'b' to the end, obtaining the final result "acb".
 *
 * Example 2:
 *
 * Input: s = "baaca", k = 3
 * Output: "aaabc"
 * Explanation:
 * In the first move, we move the 1^st character 'b' to the end, obtaining the string "aacab".
 * In the second move, we move the 3^rd character 'c' to the end, obtaining the final result "aaabc".
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= s.length <= 1000
 * 	s consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/orderly-queue/
// discuss: https://leetcode.com/problems/orderly-queue/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/orderly-queue/solutions/1446287/rust-solution/
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let ss = s.chars().chain(s.chars()).collect::<Vec<_>>();
            let mut v = ss.windows(s.len()).collect::<Vec<_>>();
            v.sort();
            v[0].iter().copied().collect()
        } else {
            let mut v = s.chars().collect::<Vec<_>>();
            v.sort();
            v.iter().collect()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0899_example_1() {
        let s = "cba".to_string();
        let k = 1;
        let result = "acb".to_string();

        assert_eq!(Solution::orderly_queue(s, k), result);
    }

    #[test]
    fn test_0899_example_2() {
        let s = "baaca".to_string();
        let k = 3;
        let result = "aaabc".to_string();

        assert_eq!(Solution::orderly_queue(s, k), result);
    }
}
