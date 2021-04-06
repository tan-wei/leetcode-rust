/**
 * [89] Gray Code
 *
 * The gray code is a binary numeral system where two successive values differ in only one bit.
 * Given an integer n representing the total number of bits in the code, return any sequence of gray code.
 * A gray code sequence must begin with 0.
 *  
 * Example 1:
 *
 * Input: n = 2
 * Output: [0,1,3,2]
 * Explanation:
 * 00 - 0
 * 0<u>1</u> - 1
 * <u>1</u>1 - 3
 * 1<u>0</u> - 2
 * [0,2,3,1] is also a valid gray code sequence.
 * 00 - 0
 * <u>1</u>0 - 2
 * 1<u>1</u> - 3
 * <u>0</u>1 - 1
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [0,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 16
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/gray-code/
// discuss: https://leetcode.com/problems/gray-code/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut x = 1;
        let mut ret = vec![0];

        for _ in 0..n {
            let mut rev = ret.iter().rev().map(|&num| num + x).collect();
            ret.append(&mut rev);
            x *= 2;
        }

        ret
    }

    pub fn gray_code_v2(n: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = vec![];
        let count = (2 as i32).pow(n as u32);
        for i in 0..count as usize {
            ret.push((i as i32) ^ (i as i32 >> 1));
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0089_example_1() {
        let n = 2;
        let result = vec![0, 1, 3, 2];

        assert_eq!(Solution::gray_code(n), result);

        assert_eq!(Solution::gray_code_v2(n), result);
    }

    #[test]
    #[ignore]
    fn test_0089_example_2() {
        let n = 1;
        let result = vec![0, 1];

        assert_eq!(Solution::gray_code(n), result);

        assert_eq!(Solution::gray_code_v2(n), result);
    }
}
