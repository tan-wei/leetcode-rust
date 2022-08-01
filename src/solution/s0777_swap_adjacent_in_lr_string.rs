/**
 * [0777] Swap Adjacent in LR String
 *
 * In a string composed of 'L', 'R', and 'X' characters, like "RXXLRXRXL", a move consists of either replacing one occurrence of "XL" with "LX", or replacing one occurrence of "RX" with "XR". Given the starting string start and the ending string end, return True if and only if there exists a sequence of moves to transform one string to the other.
 *  
 * Example 1:
 *
 * Input: start = "RXXLRXRXL", end = "XRLXXRRLX"
 * Output: true
 * Explanation: We can transform start to end following these steps:
 * RXXLRXRXL ->
 * XRXLRXRXL ->
 * XRLXRXRXL ->
 * XRLXXRRXL ->
 * XRLXXRRLX
 *
 * Example 2:
 *
 * Input: start = "X", end = "L"
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= start.length <= 10^4
 * 	start.length == end.length
 * 	Both start and end will only consist of characters in 'L', 'R', and 'X'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/swap-adjacent-in-lr-string/
// discuss: https://leetcode.com/problems/swap-adjacent-in-lr-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/swap-adjacent-in-lr-string/discuss/2264947/Rust-Simple-Loop%3A-Runtime-O(N)-Memory-O(N)
    pub fn can_transform(start: String, end: String) -> bool {
        let n = start.len();
        let start = start.chars().collect::<Vec<char>>();
        let end = end.chars().collect::<Vec<char>>();

        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        for i in 0..n {
            if start[i] != 'X' {
                v1.push((start[i], i));
            }
            if end[i] != 'X' {
                v2.push((end[i], i));
            }

            if v1.len() < v2.len() && v2[v2.len() - 1].0 == 'R' {
                return false;
            }

            if v1.len() > v2.len() && v1[v1.len() - 1].0 == 'L' {
                return false;
            }

            if v1.is_empty() && v2.is_empty() {
                continue;
            }
            if v1.len() == v2.len() && v1[v1.len() - 1].0 != v2[v2.len() - 1].0 {
                return false;
            }
        }

        v1.len() == v2.len()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0777_example_1() {
        let start = "RXXLRXRXL".to_string();
        let end = "XRLXXRRLX".to_string();
        let result = true;

        assert_eq!(Solution::can_transform(start, end), result);
    }

    #[test]
    fn test_0777_example_2() {
        let start = "X".to_string();
        let end = "L".to_string();
        let result = false;

        assert_eq!(Solution::can_transform(start, end), result);
    }
}
