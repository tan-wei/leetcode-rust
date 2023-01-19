/**
 * [0949] Largest Time for Given Digits
 *
 * Given an array arr of 4 digits, find the latest 24-hour time that can be made using each digit exactly once.
 * 24-hour times are formatted as "HH:MM", where HH is between 00 and 23, and MM is between 00 and 59. The earliest 24-hour time is 00:00, and the latest is 23:59.
 * Return the latest 24-hour time in "HH:MM" format. If no valid time can be made, return an empty string.
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,4]
 * Output: "23:41"
 * Explanation: The valid 24-hour times are "12:34", "12:43", "13:24", "13:42", "14:23", "14:32", "21:34", "21:43", "23:14", and "23:41". Of these times, "23:41" is the latest.
 *
 * Example 2:
 *
 * Input: arr = [5,5,5,5]
 * Output: ""
 * Explanation: There are no valid 24-hour times as "55:55" is not valid.
 *
 *  
 * Constraints:
 *
 * 	arr.length == 4
 * 	0 <= arr[i] <= 9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-time-for-given-digits/
// discuss: https://leetcode.com/problems/largest-time-for-given-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_time_from_digits(arr: Vec<i32>) -> String {
        let mut arr = arr;
        let mut result = Vec::new();
        Solution::dfs_helper(&mut arr, 0, &mut result);
        if let Some(max) = result.iter().max() {
            format!("{:02}:{:02}", max.0, max.1)
        } else {
            String::new()
        }
    }

    fn dfs_helper(arr: &mut [i32], i: usize, result: &mut Vec<(i32, i32)>) {
        if i == 4 {
            let (h, m) = (arr[0] * 10 + arr[1], arr[2] * 10 + arr[3]);
            if h < 24 && m < 60 {
                result.push((h, m));
            }
            return;
        }
        for j in i..4 {
            arr.swap(i, j);
            Solution::dfs_helper(arr, i + 1, result);
            arr.swap(i, j);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0949_example_1() {
        let arr = vec![1, 2, 3, 4];
        let result = "23:41".to_string();

        assert_eq!(Solution::largest_time_from_digits(arr), result);
    }

    #[test]
    fn test_0949_example_2() {
        let arr = vec![5, 5, 5, 5];
        let result = "".to_string();

        assert_eq!(Solution::largest_time_from_digits(arr), result);
    }
}
