/**
 * [1641] Count Sorted Vowel Strings
 *
 * Given an integer n, return the number of strings of length n that consist only of vowels (a, e, i, o, u) and are lexicographically sorted.
 * A string s is lexicographically sorted if for all valid i, s[i] is the same as or comes before s[i+1] in the alphabet.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: 5
 * Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: 15
 * Explanation: The 15 sorted strings that consist of vowels only are
 * ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
 * Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.
 *
 * Example 3:
 *
 * Input: n = 33
 * Output: 66045
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 50
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-sorted-vowel-strings/
// discuss: https://leetcode.com/problems/count-sorted-vowel-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp0 = [1, 2, 3, 4, 5]; // n = 1.
        let mut dp1 = [1, 3, 6, 10, 15]; // n = 2.

        for _ in 3..=n {
            for i in 1..=4 {
                dp0[i] = dp0[i - 1] + dp1[i];
            }
            std::mem::swap(&mut dp0, &mut dp1);
        }

        if n < 2 { dp0[4] } else { dp1[4] }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1641_example_1() {
        let n = 1;

        let result = 5;

        assert_eq!(Solution::count_vowel_strings(n), result);
    }

    #[test]
    fn test_1641_example_2() {
        let n = 2;

        let result = 15;

        assert_eq!(Solution::count_vowel_strings(n), result);
    }

    #[test]
    fn test_1641_example_3() {
        let n = 33;

        let result = 66045;

        assert_eq!(Solution::count_vowel_strings(n), result);
    }
}
