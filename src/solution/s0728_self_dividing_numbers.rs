/**
 * [0728] Self Dividing Numbers
 *
 * A self-dividing number is a number that is divisible by every digit it contains.
 *
 * 	For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.
 *
 * A self-dividing number is not allowed to contain the digit zero.
 * Given two integers left and right, return a list of all the self-dividing numbers in the range [left, right].
 *  
 * Example 1:
 * Input: left = 1, right = 22
 * Output: [1,2,3,4,5,6,7,8,9,11,12,15,22]
 * Example 2:
 * Input: left = 47, right = 85
 * Output: [48,55,66,77]
 *  
 * Constraints:
 *
 * 	1 <= left <= right <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/self-dividing-numbers/
// discuss: https://leetcode.com/problems/self-dividing-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = vec![];
        'outer: for num in left..=right {
            let mut n = num;
            while n > 0 {
                let re = n % 10;
                if re == 0 || num % re > 0 {
                    continue 'outer;
                }
                n /= 10;
            }
            result.push(num);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0728_example_1() {
        let left = 1;
        let right = 22;
        let result = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22];

        assert_eq!(Solution::self_dividing_numbers(left, right), result);
    }

    #[test]
    fn test_0728_example_2() {
        let left = 47;
        let right = 85;
        let result = vec![48, 55, 66, 77];

        assert_eq!(Solution::self_dividing_numbers(left, right), result);
    }
}
