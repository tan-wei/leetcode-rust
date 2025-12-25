/**
 * [2287] Rearrange Characters to Make Target String
 *
 * You are given two 0-indexed strings s and target. You can take some letters from s and rearrange them to form new strings.
 * Return the maximum number of copies of target that can be formed by taking letters from s and rearranging them.
 *  
 * Example 1:
 *
 * Input: s = "ilovecodingonleetcode", target = "code"
 * Output: 2
 * Explanation:
 * For the first copy of "code", take the letters at indices 4, 5, 6, and 7.
 * For the second copy of "code", take the letters at indices 17, 18, 19, and 20.
 * The strings that are formed are "ecod" and "code" which can both be rearranged into "code".
 * We can make at most two copies of "code", so we return 2.
 *
 * Example 2:
 *
 * Input: s = "abcba", target = "abc"
 * Output: 1
 * Explanation:
 * We can make one copy of "abc" by taking the letters at indices 0, 1, and 2.
 * We can make at most one copy of "abc", so we return 1.
 * Note that while there is an extra 'a' and 'b' at indices 3 and 4, we cannot reuse the letter 'c' at index 2, so we cannot make a second copy of "abc".
 *
 * Example 3:
 *
 * Input: s = "abbaccaddaeea", target = "aaaaa"
 * Output: 1
 * Explanation:
 * We can make one copy of "aaaaa" by taking the letters at indices 0, 3, 6, 9, and 12.
 * We can make at most one copy of "aaaaa", so we return 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	1 <= target.length <= 10
 * 	s and target consist of lowercase English letters.
 *
 *  
 * Note: This question is the same as <a href="https://leetcode.com/problems/maximum-number-of-balloons/description/" target="_blank"> 1189: Maximum Number of Balloons.</a>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rearrange-characters-to-make-target-string/
// discuss: https://leetcode.com/problems/rearrange-characters-to-make-target-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        target
            .bytes()
            .fold([0; 26], |mut f, b| {
                f[(b - b'a') as usize] += 1;
                f
            })
            .iter()
            .zip(s.bytes().fold([0; 26], |mut f, b| {
                f[(b - b'a') as usize] += 1;
                f
            }))
            .flat_map(|(tf, sf)| (*tf > 0).then(|| sf / tf))
            .min()
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2287_example_1() {
        let s = "ilovecodingonleetcode".to_string();
        let target = "code".to_string();

        let result = 2;

        assert_eq!(Solution::rearrange_characters(s, target), result);
    }

    #[test]
    fn test_2287_example_2() {
        let s = "abcba".to_string();
        let target = "abc".to_string();

        let result = 1;

        assert_eq!(Solution::rearrange_characters(s, target), result);
    }

    #[test]
    fn test_2287_example_3() {
        let s = "abbaccaddaeea".to_string();
        let target = "aaaaa".to_string();

        let result = 1;

        assert_eq!(Solution::rearrange_characters(s, target), result);
    }
}
