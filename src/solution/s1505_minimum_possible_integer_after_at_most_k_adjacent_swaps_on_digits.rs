/**
 * [1505] Minimum Possible Integer After at Most K Adjacent Swaps On Digits
 *
 * You are given a string num representing the digits of a very large integer and an integer k. You are allowed to swap any two adjacent digits of the integer at most k times.
 * Return the minimum integer you can obtain also as a string.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/17/q4_1.jpg" style="width: 500px; height: 40px;" />
 * Input: num = "4321", k = 4
 * Output: "1342"
 * Explanation: The steps to obtain the minimum integer from 4321 with 4 adjacent swaps are shown.
 *
 * Example 2:
 *
 * Input: num = "100", k = 1
 * Output: "010"
 * Explanation: It's ok for the output to have leading zeros, but the input is guaranteed not to have any leading zeros.
 *
 * Example 3:
 *
 * Input: num = "36789", k = 1000
 * Output: "36789"
 * Explanation: We can keep the number without any swaps.
 *
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 3 * 10^4
 * 	num consists of only digits and does not contain leading zeros.
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/
// discuss: https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-possible-integer-after-at-most-k-adjacent-swaps-on-digits/solutions/3162681/just-a-runnable-solution/
    pub fn min_integer(num: String, k: i32) -> String {
        let mut k = k as usize;
        let num = num.into_bytes();
        let n = num.len();
        let mut result = Vec::with_capacity(n);
        let mut q = vec![n; 10];
        for (i, &item) in num.iter().enumerate() {
            let d = (item - b'0') as usize;
            if q[d] == n {
                q[d] = i;
            }
        }
        let mut used = vec![false; n];
        let mut q_used = vec![0; 10];

        for _ in 0..n {
            for d in 0..10_usize {
                if q[d] == n {
                    continue;
                }
                let c = q[d] - q_used[d];
                if c <= k {
                    k -= c;
                    result.push(b'0' + d as u8);
                    used[q[d]] = true;
                    for d1 in 0..10_usize {
                        if q[d1] > q[d] {
                            q_used[d1] += 1;
                        }
                    }
                    while q[d] < n {
                        if used[q[d]] {
                            q_used[d] += 1;
                        }
                        q[d] += 1;
                        let &c = num.get(q[d]).unwrap_or(&0_u8);
                        if c == b'0' + d as u8 {
                            break;
                        }
                    }
                    break;
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1505_example_1() {
        let num = "4321".to_string();
        let k = 4;

        let result = "1342".to_string();

        assert_eq!(Solution::min_integer(num, k), result);
    }

    #[test]
    fn test_1505_example_2() {
        let num = "36789".to_string();
        let k = 1000;

        let result = "36789".to_string();

        assert_eq!(Solution::min_integer(num, k), result);
    }

    #[test]
    fn test_1505_example_3() {
        let num = "100".to_string();
        let k = 1;

        let result = "010".to_string();

        assert_eq!(Solution::min_integer(num, k), result);
    }
}
