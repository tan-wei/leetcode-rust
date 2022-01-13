/**
 * [0483] Smallest Good Base
 *
 * Given an integer n represented as a string, return the smallest good base of n.
 * We call k >= 2 a good base of n, if all digits of n base k are 1's.
 *  
 * Example 1:
 *
 * Input: n = "13"
 * Output: "3"
 * Explanation: 13 base 3 is 111.
 *
 * Example 2:
 *
 * Input: n = "4681"
 * Output: "8"
 * Explanation: 4681 base 8 is 11111.
 *
 * Example 3:
 *
 * Input: n = "1000000000000000000"
 * Output: "999999999999999999"
 * Explanation: 1000000000000000000 base 999999999999999999 is 11.
 *
 *  
 * Constraints:
 *
 * 	n is an integer in the range [3, 10^18].
 * 	n does not contain any leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-good-base/
// discuss: https://leetcode.com/problems/smallest-good-base/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n = n.parse::<i64>().unwrap();
        let longest = (n as f64).log(2.0) as u32 + 1;

        for m in (2..=longest).rev() {
            let k: i64 = f64::powf(n as f64, 1.0 / (m as f64 - 1.0)) as i64;
            if Self::check(n, k, m) {
                return k.to_string();
            }
        }

        (n - 1).to_string()
    }

    fn check(n: i64, k: i64, m: u32) -> bool {
        let k_pow_m = (k as i128).pow(m);
        (k_pow_m - 1) % (k as i128 - 1) == 0 && n as i128 == (k_pow_m - 1) / (k as i128 - 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0483_example_1() {
        let n = "13".to_string();
        let result = "3".to_string();

        assert_eq!(Solution::smallest_good_base(n), result);
    }

    #[test]
    fn test_0483_example_2() {
        let n = "4681".to_string();
        let result = "8".to_string();

        assert_eq!(Solution::smallest_good_base(n), result);
    }

    #[test]
    fn test_0483_example_3() {
        let n = "1000000000000000000".to_string();
        let result = "999999999999999999".to_string();

        assert_eq!(Solution::smallest_good_base(n), result);
    }
}
