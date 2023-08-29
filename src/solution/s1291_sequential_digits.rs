/**
 * [1291] Sequential Digits
 *
 * An integer has sequential digits if and only if each digit in the number is one more than the previous digit.
 * Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
 *
 * Example 1:
 * Input: low = 100, high = 300
 * Output: [123,234]
 * Example 2:
 * Input: low = 1000, high = 13000
 * Output: [1234,2345,3456,4567,5678,6789,12345]
 *
 * Constraints:
 *
 * 	10 <= low <= high <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sequential-digits/
// discuss: https://leetcode.com/problems/sequential-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mold = "123456789".as_bytes();

        let low_len = low.to_string().len();
        let high_len = high.to_string().len();

        let mut result = vec![];
        for i in low_len..high_len + 1 {
            let t = mold
                .windows(i)
                .map(|n| {
                    String::from_utf8(n.to_vec())
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                })
                .filter(|&n| n >= low && n <= high)
                .collect::<Vec<_>>();

            result.push(t);
        }

        result.into_iter().flatten().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1291_example_1() {
        let low = 100;
        let high = 300;
        let result = vec![123, 234];

        assert_eq!(Solution::sequential_digits(low, high), result);
    }

    #[test]
    fn test_1291_example_2() {
        let low = 1000;
        let high = 13000;
        let result = vec![1234, 2345, 3456, 4567, 5678, 6789, 12345];

        assert_eq!(Solution::sequential_digits(low, high), result);
    }
}
