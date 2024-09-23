/**
 * [1694] Reformat Phone Number
 *
 * You are given a phone number as a string number. number consists of digits, spaces ' ', and/or dashes '-'.
 * You would like to reformat the phone number in a certain manner. Firstly, remove all spaces and dashes. Then, group the digits from left to right into blocks of length 3 until there are 4 or fewer digits. The final digits are then grouped as follows:
 *
 * 	2 digits: A single block of length 2.
 * 	3 digits: A single block of length 3.
 * 	4 digits: Two blocks of length 2 each.
 *
 * The blocks are then joined by dashes. Notice that the reformatting process should never produce any blocks of length 1 and produce at most two blocks of length 2.
 * Return the phone number after formatting.
 *  
 * Example 1:
 *
 * Input: number = "1-23-45 6"
 * Output: "123-456"
 * Explanation: The digits are "123456".
 * Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
 * Step 2: There are 3 digits remaining, so put them in a single block of length 3. The 2nd block is "456".
 * Joining the blocks gives "123-456".
 *
 * Example 2:
 *
 * Input: number = "123 4-567"
 * Output: "123-45-67"
 * Explanation: The digits are "1234567".
 * Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
 * Step 2: There are 4 digits left, so split them into two blocks of length 2. The blocks are "45" and "67".
 * Joining the blocks gives "123-45-67".
 *
 * Example 3:
 *
 * Input: number = "123 4-5678"
 * Output: "123-456-78"
 * Explanation: The digits are "12345678".
 * Step 1: The 1st block is "123".
 * Step 2: The 2nd block is "456".
 * Step 3: There are 2 digits left, so put them in a single block of length 2. The 3rd block is "78".
 * Joining the blocks gives "123-456-78".
 *
 *  
 * Constraints:
 *
 * 	2 <= number.length <= 100
 * 	number consists of digits and the characters '-' and ' '.
 * 	There are at least two digits in number.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reformat-phone-number/
// discuss: https://leetcode.com/problems/reformat-phone-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let digits: Vec<_> = number.chars().filter(|c| c.is_digit(10)).collect();

        let mut chunked: Vec<_> = digits.chunks(3).map(|group| group.to_vec()).collect();

        let n = chunked.len();
        if chunked[n - 1].len() == 1 {
            let tmp = chunked[n - 2].pop().unwrap();
            chunked[n - 1].insert(0, tmp);
        }

        chunked
            .into_iter()
            .map(|group| group.into_iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("-")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1694_example_1() {
        let number = "1-23-45 6".to_string();

        let result = "123-456".to_string();

        assert_eq!(Solution::reformat_number(number), result);
    }

    #[test]
    fn test_1694_example_2() {
        let number = "123 4-567".to_string();

        let result = "123-45-67".to_string();

        assert_eq!(Solution::reformat_number(number), result);
    }

    #[test]
    fn test_1694_example_3() {
        let number = "123 4-5678".to_string();

        let result = "123-456-78".to_string();

        assert_eq!(Solution::reformat_number(number), result);
    }
}
