/**
 * [1598] Crawler Log Folder
 *
 * The Leetcode file system keeps a log each time some user performs a change folder operation.
 * The operations are described below:
 *
 * 	"../" : Move to the parent folder of the current folder. (If you are already in the main folder, remain in the same folder).
 * 	"./" : Remain in the same folder.
 * 	"x/" : Move to the child folder named x (This folder is guaranteed to always exist).
 *
 * You are given a list of strings logs where logs[i] is the operation performed by the user at the i^th step.
 * The file system starts in the main folder, then the operations in logs are performed.
 * Return the minimum number of operations needed to go back to the main folder after the change folder operations.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/sample_11_1957.png" style="width: 775px; height: 151px;" />
 *
 * Input: logs = ["d1/","d2/","../","d21/","./"]
 * Output: 2
 * Explanation: Use this change folder operation "../" 2 times and go back to the main folder.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/sample_22_1957.png" style="width: 600px; height: 270px;" />
 *
 * Input: logs = ["d1/","d2/","./","d3/","../","d31/"]
 * Output: 3
 *
 * Example 3:
 *
 * Input: logs = ["d1/","../","../","../"]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= logs.length <= 10^3
 * 	2 <= logs[i].length <= 10
 * 	logs[i] contains lowercase English letters, digits, '.', and '/'.
 * 	logs[i] follows the format described in the statement.
 * 	Folder names consist of lowercase English letters and digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/crawler-log-folder/
// discuss: https://leetcode.com/problems/crawler-log-folder/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        logs.iter()
            .fold(0_i32, |level, log| match (level, log.as_str()) {
                (_, "./") | (0, "../") => level,
                (_, "../") => level - 1,
                _ => level + 1,
            })
            .abs()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1598_example_1() {
        let logs = vec_string!["d1/", "d2/", "../", "d21/", "./"];

        let result = 2;

        assert_eq!(Solution::min_operations(logs), result);
    }

    #[test]
    fn test_1598_example_2() {
        let logs = vec_string!["d1/", "d2/", "./", "d3/", "../", "d31/"];

        let result = 3;

        assert_eq!(Solution::min_operations(logs), result);
    }

    #[test]
    fn test_1598_example_3() {
        let logs = vec_string!["d1/", "../", "../", "../"];

        let result = 0;

        assert_eq!(Solution::min_operations(logs), result);
    }
}
