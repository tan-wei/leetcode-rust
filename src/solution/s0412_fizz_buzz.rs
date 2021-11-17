/**
 * [0412] Fizz Buzz
 *
 * Given an integer n, return a string array answer (1-indexed) where:
 *
 * 	answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
 * 	answer[i] == "Fizz" if i is divisible by 3.
 * 	answer[i] == "Buzz" if i is divisible by 5.
 * 	answer[i] == i if non of the above conditions are true.
 *
 *  
 * Example 1:
 * Input: n = 3
 * Output: ["1","2","Fizz"]
 * Example 2:
 * Input: n = 5
 * Output: ["1","2","Fizz","4","Buzz"]
 * Example 3:
 * Input: n = 15
 * Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fizz-buzz/
// discuss: https://leetcode.com/problems/fizz-buzz/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut answer = Vec::new();
        for i in 1..=n as usize {
            answer.push(if (i % 15 == 0) {
                "FizzBuzz".to_string()
            } else {
                if (i % 3 == 0) {
                    "Fizz".to_string()
                } else {
                    if (i % 5 == 0) {
                        "Buzz".to_string()
                    } else {
                        i.to_string()
                    }
                }
            });
        }
        answer
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0412_example_1() {
        let n = 3;
        let result = vec_string!["1", "2", "Fizz"];

        assert_eq!(Solution::fizz_buzz(n), result);
    }

    #[test]
    fn test_0412_example_2() {
        let n = 5;
        let result = vec_string!["1", "2", "Fizz", "4", "Buzz"];

        assert_eq!(Solution::fizz_buzz(n), result);
    }

    #[test]
    fn test_0412_example_3() {
        let n = 15;
        let result = vec_string![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz"
        ];

        assert_eq!(Solution::fizz_buzz(n), result);
    }
}
