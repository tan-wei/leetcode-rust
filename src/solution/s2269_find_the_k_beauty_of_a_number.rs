/**
 * [2269] Find the K-Beauty of a Number
 *
 * The k-beauty of an integer num is defined as the number of substrings of num when it is read as a string that meet the following conditions:
 *
 * 	It has a length of k.
 * 	It is a divisor of num.
 *
 * Given integers num and k, return the k-beauty of num.
 * Note:
 *
 * 	Leading zeros are allowed.
 * 	0 is not a divisor of any value.
 *
 * A substring is a contiguous sequence of characters in a string.
 *  
 * Example 1:
 *
 * Input: num = 240, k = 2
 * Output: 2
 * Explanation: The following are the substrings of num of length k:
 * - "24" from "<u>24</u>0": 24 is a divisor of 240.
 * - "40" from "2<u>40</u>": 40 is a divisor of 240.
 * Therefore, the k-beauty is 2.
 *
 * Example 2:
 *
 * Input: num = 430043, k = 2
 * Output: 2
 * Explanation: The following are the substrings of num of length k:
 * - "43" from "<u>43</u>0043": 43 is a divisor of 430043.
 * - "30" from "4<u>30</u>043": 30 is not a divisor of 430043.
 * - "00" from "43<u>00</u>43": 0 is not a divisor of 430043.
 * - "04" from "430<u>04</u>3": 4 is not a divisor of 430043.
 * - "43" from "4300<u>43</u>": 43 is a divisor of 430043.
 * Therefore, the k-beauty is 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^9
 * 	1 <= k <= num.length (taking num as a string)
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-k-beauty-of-a-number/
// discuss: https://leetcode.com/problems/find-the-k-beauty-of-a-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let c = format!("{num}");
        (0..c.len())
            .filter_map(|i| {
                let d = c.get(i..i + k as usize)?.parse::<i32>().unwrap();
                if d != 0 && num % d == 0 {
                    Some(())
                } else {
                    None
                }
            })
            .count() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2269_example_1() {
        let num = 240;
        let k = 2;

        let result = 2;

        assert_eq!(Solution::divisor_substrings(num, k), result);
    }

    #[test]
    fn test_2269_example_2() {
        let num = 430043;
        let k = 2;

        let result = 2;

        assert_eq!(Solution::divisor_substrings(num, k), result);
    }
}
