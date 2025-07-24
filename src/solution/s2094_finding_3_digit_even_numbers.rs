/**
 * [2094] Finding 3-Digit Even Numbers
 *
 * You are given an integer array digits, where each element is a digit. The array may contain duplicates.
 * You need to find all the unique integers that follow the given requirements:
 *
 * 	The integer consists of the concatenation of three elements from digits in any arbitrary order.
 * 	The integer does not have leading zeros.
 * 	The integer is even.
 *
 * For example, if the given digits were [1, 2, 3], integers 132 and 312 follow the requirements.
 * Return a sorted array of the unique integers.
 *  
 * Example 1:
 *
 * Input: digits = [2,1,3,0]
 * Output: [102,120,130,132,210,230,302,310,312,320]
 * Explanation: All the possible integers that follow the requirements are in the output array.
 * Notice that there are no odd integers or integers with leading zeros.
 *
 * Example 2:
 *
 * Input: digits = [2,2,8,8,2]
 * Output: [222,228,282,288,822,828,882]
 * Explanation: The same digit can be used as many times as it appears in digits.
 * In this example, the digit 8 is used twice each time in 288, 828, and 882.
 *
 * Example 3:
 *
 * Input: digits = [3,7,5]
 * Output: []
 * Explanation: No even integers can be formed using the given digits.
 *
 *  
 * Constraints:
 *
 * 	3 <= digits.length <= 100
 * 	0 <= digits[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/finding-3-digit-even-numbers/
// discuss: https://leetcode.com/problems/finding-3-digit-even-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let (mut count, mut result) = (vec![0; 10], vec![]);

        digits.into_iter().for_each(|x| {
            count[x as usize] += 1;
        });

        for i in 1..10 {
            if count[i] == 0 {
                continue;
            }
            count[i] -= 1;
            for j in 0..10 {
                if count[j] == 0 {
                    continue;
                }
                count[j] -= 1;
                for k in (0..5).map(|x| x * 2) {
                    if count[k] == 0 {
                        continue;
                    }
                    count[k] -= 1;
                    result.push((i * 100 + j * 10 + k) as i32);
                    count[k] += 1;
                }
                count[j] += 1;
            }
            count[i] += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2094_example_1() {
        let digits = vec![2, 1, 3, 0];

        let result = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];

        assert_eq!(Solution::find_even_numbers(digits), result);
    }

    #[test]
    fn test_2094_example_2() {
        let digits = vec![2, 2, 8, 8, 2];

        let result = vec![222, 228, 282, 288, 822, 828, 882];

        assert_eq!(Solution::find_even_numbers(digits), result);
    }

    #[test]
    fn test_2094_example_3() {
        let digits = vec![3, 7, 5];

        let result = vec![];

        assert_eq!(Solution::find_even_numbers(digits), result);
    }
}
