/**
 * [1652] Defuse the Bomb
 *
 * You have a bomb to defuse, and your time is running out! Your informer will provide you with a circular array code of length of n and a key k.
 * To decrypt the code, you must replace every number. All the numbers are replaced simultaneously.
 *
 * 	If k > 0, replace the i^th number with the sum of the next k numbers.
 * 	If k < 0, replace the i^th number with the sum of the previous k numbers.
 * 	If k == 0, replace the i^th number with 0.
 *
 * As code is circular, the next element of code[n-1] is code[0], and the previous element of code[0] is code[n-1].
 * Given the circular array code and an integer key k, return the decrypted code to defuse the bomb!
 *  
 * Example 1:
 *
 * Input: code = [5,7,1,4], k = 3
 * Output: [12,10,16,13]
 * Explanation: Each number is replaced by the sum of the next 3 numbers. The decrypted code is [7+1+4, 1+4+5, 4+5+7, 5+7+1]. Notice that the numbers wrap around.
 *
 * Example 2:
 *
 * Input: code = [1,2,3,4], k = 0
 * Output: [0,0,0,0]
 * Explanation: When k is zero, the numbers are replaced by 0.
 *
 * Example 3:
 *
 * Input: code = [2,4,9,3], k = -2
 * Output: [12,5,6,13]
 * Explanation: The decrypted code is [3+9, 2+3, 4+2, 9+4]. Notice that the numbers wrap around again. If k is negative, the sum is of the previous numbers.
 *
 *  
 * Constraints:
 *
 * 	n == code.length
 * 	1 <= n <= 100
 * 	1 <= code[i] <= 100
 * 	-(n - 1) <= k <= n - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/defuse-the-bomb/
// discuss: https://leetcode.com/problems/defuse-the-bomb/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len = code.len() as i32;
        if k == 0 {
            vec![0; code.len()]
        } else if k < 0 {
            (0..len)
                .map(|index| {
                    (index + k..index)
                        .map(|i| code[i.rem_euclid(len) as usize])
                        .sum()
                })
                .collect()
        } else {
            (0..len)
                .map(|index| {
                    (index + 1..=index + k)
                        .map(|i| code[i.rem_euclid(len) as usize])
                        .sum()
                })
                .collect()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1652_example_1() {
        let code = vec![5, 7, 1, 4];
        let k = 3;

        let result = vec![12, 10, 16, 13];

        assert_eq!(Solution::decrypt(code, k), result);
    }

    #[test]
    fn test_1652_example_2() {
        let code = vec![1, 2, 3, 4];
        let k = 0;

        let result = vec![0, 0, 0, 0];

        assert_eq!(Solution::decrypt(code, k), result);
    }

    #[test]
    fn test_1652_example_3() {
        let code = vec![2, 4, 9, 3];
        let k = -2;

        let result = vec![12, 5, 6, 13];

        assert_eq!(Solution::decrypt(code, k), result);
    }
}
