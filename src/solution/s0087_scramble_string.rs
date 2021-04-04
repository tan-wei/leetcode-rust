/**
 * [87] Scramble String
 *
 * We can scramble a string s to get a string t using the following algorithm:
 * <ol>
 * 	If the length of the string is 1, stop.
 * 	If the length of the string is > 1, do the following:
 *
 * 		Split the string into two non-empty substrings at a random index, i.e., if the string is s, divide it to x and y where s = x + y.
 * 		Randomly decide to swap the two substrings or to keep them in the same order. i.e., after this step, s may become s = x + y or s = y + x.
 * 		Apply step 1 recursively on each of the two substrings x and y.
 *
 *
 * </ol>
 * Given two strings s1 and s2 of the same length, return true if s2 is a scrambled string of s1, otherwise, return false.
 *  
 * Example 1:
 *
 * Input: s1 = "great", s2 = "rgeat"
 * Output: true
 * Explanation: One possible scenario applied on s1 is:
 * "great" --> "gr/eat" // divide at random index.
 * "gr/eat" --> "gr/eat" // random decision is not to swap the two substrings and keep them in order.
 * "gr/eat" --> "g/r / e/at" // apply the same algorithm recursively on both substrings. divide at ranom index each of them.
 * "g/r / e/at" --> "r/g / e/at" // random decision was to swap the first substring and to keep the second substring in the same order.
 * "r/g / e/at" --> "r/g / e/ a/t" // again apply the algorithm recursively, divide "at" to "a/t".
 * "r/g / e/ a/t" --> "r/g / e/ a/t" // random decision is to keep both substrings in the same order.
 * The algorithm stops now and the result string is "rgeat" which is s2.
 * As there is one possible scenario that led s1 to be scrambled to s2, we return true.
 *
 * Example 2:
 *
 * Input: s1 = "abcde", s2 = "caebd"
 * Output: false
 *
 * Example 3:
 *
 * Input: s1 = "a", s2 = "a"
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	s1.length == s2.length
 * 	1 <= s1.length <= 30
 * 	s1 and s2 consist of lower-case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/scramble-string/
// discuss: https://leetcode.com/problems/scramble-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        fn helper<'a>(
            xs: &'a [u8],
            ys: &'a [u8],
            memo: &mut std::collections::HashMap<(&'a [u8], &'a [u8]), bool>,
        ) -> bool {
            if let Some(&b) = memo.get(&(xs, ys)) {
                return b;
            }
            if xs.len() == 1 && ys.len() == 1 {
                let ans = xs[0] == ys[0];
                memo.insert((xs, ys), ans);
                return ans;
            }
            for i in 1..xs.len() {
                let j = ys.len() - i;
                if helper(&xs[..i], &ys[..i], memo) && helper(&xs[i..], &ys[i..], memo)
                    || helper(&xs[..i], &ys[j..], memo) && helper(&xs[i..], &ys[..j], memo)
                {
                    memo.insert((xs, ys), true);
                    return true;
                }
            }
            memo.insert((xs, ys), false);
            false
        }

        let xs = s1.as_bytes();
        let ys = s2.as_bytes();
        let mut memo = std::collections::HashMap::new();

        helper(xs, ys, &mut memo)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0087_example_1() {
        let s1 = "great".to_string();
        let s2 = "rgeat".to_string();
        let result = true;

        assert_eq!(Solution::is_scramble(s1, s2), result);
    }

    #[test]
    fn test_0087_example_2() {
        let s1 = "abcde".to_string();
        let s2 = "caebd".to_string();
        let result = false;

        assert_eq!(Solution::is_scramble(s1, s2), result);
    }

    #[test]
    fn test_0087_example_3() {
        let s1 = "a".to_string();
        let s2 = "a".to_string();
        let result = true;

        assert_eq!(Solution::is_scramble(s1, s2), result);
    }
}
