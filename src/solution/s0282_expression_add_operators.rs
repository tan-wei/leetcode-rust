/**
 * [282] Expression Add Operators
 *
 * Given a string num that contains only digits and an integer target, return all possibilities to add the binary operators '+', '-', or '*' between the digits of num so that the resultant expression evaluates to the target value.
 *  
 * Example 1:
 * Input: num = "123", target = 6
 * Output: ["1*2*3","1+2+3"]
 * Example 2:
 * Input: num = "232", target = 8
 * Output: ["2*3+2","2+3*2"]
 * Example 3:
 * Input: num = "105", target = 5
 * Output: ["1*0+5","10-5"]
 * Example 4:
 * Input: num = "00", target = 0
 * Output: ["0*0","0+0","0-0"]
 * Example 5:
 * Input: num = "3456237490", target = 9191
 * Output: []
 *  
 * Constraints:
 *
 * 	1 <= num.length <= 10
 * 	num consists of only digits.
 * 	-2^31 <= target <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/expression-add-operators/
// discuss: https://leetcode.com/problems/expression-add-operators/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        fn helper(
            ans: &mut Vec<String>,
            path: &mut [u8],
            mut path_index: i32,
            chars: &[u8],
            char_index: i32,
            left: i64,
            cur: i64,
            target: i32,
        ) {
            if char_index == chars.len() as i32 {
                if left + cur == target as i64 {
                    ans.push(
                        std::str::from_utf8(&path[0..path_index as usize])
                            .unwrap()
                            .to_owned(),
                    );
                }
                return;
            }
            let mut n = 0i64;
            let sign_index = path_index;
            path_index += 1;
            for i in char_index..chars.len() as i32 {
                path[path_index as usize] = chars[i as usize];
                path_index += 1;
                n = n * 10 + (chars[i as usize] - b'0') as i64;
                path[sign_index as usize] = b'+';
                helper(ans, path, path_index, chars, i + 1, left + cur, n, target);
                path[sign_index as usize] = b'-';
                helper(ans, path, path_index, chars, i + 1, left + cur, -n, target);
                path[sign_index as usize] = b'*';
                helper(ans, path, path_index, chars, i + 1, left, cur * n, target);
                if n == 0 {
                    break;
                }
            }
        }

        let n = num.len();
        let v = num.as_bytes();
        let mut path = vec![0; n + n];
        let mut res = vec![];
        let mut x = 0i64;
        for i in 0..n {
            x = x * 10 + (v[i] - b'0') as i64;
            path[i] = v[i];
            helper(
                &mut res,
                &mut path,
                (i + 1) as i32,
                v,
                (i + 1) as i32,
                0,
                x,
                target,
            );
            if x == 0 {
                break;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0282_example_1() {
        let num = "123".to_string();
        let target = 6;
        let result = vec_string!["1*2*3", "1+2+3"];

        assert_eq_sorted!(Solution::add_operators(num, target), result);
    }

    #[test]
    fn test_0282_example_2() {
        let num = "232".to_string();
        let target = 8;
        let result = vec_string!["2*3+2", "2+3*2"];

        assert_eq_sorted!(Solution::add_operators(num, target), result);
    }

    #[test]
    fn test_0282_example_3() {
        let num = "105".to_string();
        let target = 5;
        let result = vec_string!["1*0+5", "10-5"];

        assert_eq_sorted!(Solution::add_operators(num, target), result);
    }

    #[test]
    fn test_0282_example_4() {
        let num = "00".to_string();
        let target = 0;
        let result = vec_string!["0*0", "0+0", "0-0"];

        assert_eq_sorted!(Solution::add_operators(num, target), result);
    }

    #[test]
    fn test_0282_example_5() {
        let num = "3456237490".to_string();
        let target = 9191;
        let result: Vec<String> = vec_string![];

        assert_eq_sorted!(Solution::add_operators(num, target), result);
    }
}
