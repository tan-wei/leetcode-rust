/**
 * [1349] Maximum Students Taking Exam
 *
 * Given a m * n matrix seats  that represent seats distributions in a classroom. If a seat is broken, it is denoted by '#' character otherwise it is denoted by a '.' character.
 * Students can see the answers of those sitting next to the left, right, upper left and upper right, but he cannot see the answers of the student sitting directly in front or behind him. Return the maximum number of students that can take the exam together without any cheating being possible.
 * Students must be placed in seats in good condition.
 *  
 * Example 1:
 * <img height="200" src="https://assets.leetcode.com/uploads/2020/01/29/image.png" width="339" />
 * Input: seats = [["#",".","#","#",".","#"],
 *                 [".","#","#","#","#","."],
 *                 ["#",".","#","#",".","#"]]
 * Output: 4
 * Explanation: Teacher can place 4 students in available seats so they don't cheat on the exam.
 *
 * Example 2:
 *
 * Input: seats = [[".","#"],
 *                 ["#","#"],
 *                 ["#","."],
 *                 ["#","#"],
 *                 [".","#"]]
 * Output: 3
 * Explanation: Place all students in available seats.
 *
 * Example 3:
 *
 * Input: seats = [["#",".",".",".","#"],
 *                 [".","#",".","#","."],
 *                 [".",".","#",".","."],
 *                 [".","#",".","#","."],
 *                 ["#",".",".",".","#"]]
 * Output: 10
 * Explanation: Place students in available seats in column 1, 3 and 5.
 *
 *  
 * Constraints:
 *
 * 	seats contains only characters '.'<font face="sans-serif, Arial, Verdana, Trebuchet MS"> and</font>'#'.
 * 	m == seats.length
 * 	n == seats[i].length
 * 	1 <= m <= 8
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-students-taking-exam/
// discuss: https://leetcode.com/problems/maximum-students-taking-exam/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-students-taking-exam/solutions/3100472/just-a-runnable-solution/
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        let m = seats.len();
        let n = seats[0].len();
        let mut validity = vec![0; m + 1];
        for i in 0..m {
            let mut rowvalid = 0;
            for j in 0..n {
                if seats[i][j] == '.' {
                    rowvalid += 1 << j;
                }
            }
            validity[i + 1] = rowvalid;
        }
        let mut dp = vec![vec![-1; (1 << n) + 1]; m + 1];
        dp[0][0] = 0;
        for i in 1..=m {
            let valid = validity[i];
            for j in 0..(1 << n) {
                if (j & valid) != j {
                    continue;
                }
                if (j & (j >> 1)) != 0 {
                    continue;
                }
                for k in 0..(1 << n) {
                    if dp[i - 1][k] == -1 {
                        continue;
                    }
                    if (j & (k >> 1)) != 0 || (k & (j >> 1)) != 0 {
                        continue;
                    }
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + j.count_ones() as i32);
                }
            }
        }
        *dp[m].iter().max().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1349_example_1() {
        let seats = vec![
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['.', '#', '#', '#', '#', '.'],
            vec!['#', '.', '#', '#', '.', '#'],
        ];

        let result = 4;

        assert_eq!(Solution::max_students(seats), result);
    }

    #[test]
    fn test_1349_example_2() {
        let seats = vec![
            vec!['.', '#'],
            vec!['#', '#'],
            vec!['#', '.'],
            vec!['#', '#'],
            vec!['.', '#'],
        ];

        let result = 3;

        assert_eq!(Solution::max_students(seats), result);
    }

    #[test]
    fn test_1349_example_3() {
        let seats = vec![
            vec!['#', '.', '.', '.', '#'],
            vec!['.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#'],
        ];

        let result = 10;

        assert_eq!(Solution::max_students(seats), result);
    }
}
