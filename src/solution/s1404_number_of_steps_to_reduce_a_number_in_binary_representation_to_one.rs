/**
 * [1404] Number of Steps to Reduce a Number in Binary Representation to One
 *
 * Given the binary representation of an integer as a string s, return the number of steps to reduce it to 1 under the following rules:
 *
 *
 * 	If the current number is even, you have to divide it by 2.
 *
 *
 * 	If the current number is odd, you have to add 1 to it.
 *
 *
 * It is guaranteed that you can always reach one for all test cases.
 *  
 * Example 1:
 *
 * Input: s = "1101"
 * Output: 6
 * Explanation: "1101" corressponds to number 13 in their decimal representation.
 * Step 1) 13 is odd, add 1 and obtain 14.
 * Step 2) 14 is even, divide by 2 and obtain 7.
 * Step 3) 7 is odd, add 1 and obtain 8.
 * Step 4) 8 is even, divide by 2 and obtain 4.  
 * Step 5) 4 is even, divide by 2 and obtain 2.
 * Step 6) 2 is even, divide by 2 and obtain 1.  
 *
 * Example 2:
 *
 * Input: s = "10"
 * Output: 1
 * Explanation: "10" corressponds to number 2 in their decimal representation.
 * Step 1) 2 is even, divide by 2 and obtain 1.  
 *
 * Example 3:
 *
 * Input: s = "1"
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of characters '0' or '1'
 * 	s[0] == '1'
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
// discuss: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut result = 0;
        let mut s = s
            .chars()
            .into_iter()
            .map(|v| if v == '1' { 1 } else { 0 })
            .collect::<std::collections::VecDeque<u8>>();
        while s.len() > 1 {
            let li = s.len() - 1;
            if s[li] == 1 {
                let mut stop = false;
                s[li] = 0;
                for i in (0..li).rev() {
                    if s[i] == 0 {
                        s[i] = 1;
                        stop = true;
                        break;
                    } else {
                        s[i] = 0;
                    }
                }

                if !stop {
                    s.push_front(1);
                }
            } else {
                s.pop_back();
            }
            result += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1404_example_1() {
        let s = "1101".to_string();

        let result = 6;

        assert_eq!(Solution::num_steps(s), result);
    }

    #[test]
    fn test_1404_example_2() {
        let s = "10".to_string();

        let result = 1;

        assert_eq!(Solution::num_steps(s), result);
    }

    #[test]
    fn test_1404_example_3() {
        let s = "1".to_string();

        let result = 0;

        assert_eq!(Solution::num_steps(s), result);
    }
}
