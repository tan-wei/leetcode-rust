/**
 * [2019] The Score of Students Solving Math Expression
 *
 * You are given a string s that contains digits 0-9, addition symbols '+', and multiplication symbols '*' only, representing a valid math expression of single digit numbers (e.g., 3+5*2). This expression was given to n elementary school students. The students were instructed to get the answer of the expression by following this order of operations:
 * <ol>
 * 	Compute multiplication, reading from left to right; Then,
 * 	Compute addition, reading from left to right.
 * </ol>
 * You are given an integer array answers of length n, which are the submitted answers of the students in no particular order. You are asked to grade the answers, by following these rules:
 *
 * 	If an answer equals the correct answer of the expression, this student will be rewarded 5 points;
 * 	Otherwise, if the answer could be interpreted as if the student applied the operators in the wrong order but had correct arithmetic, this student will be rewarded 2 points;
 * 	Otherwise, this student will be rewarded 0 points.
 *
 * Return the sum of the points of the students.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/09/17/student_solving_math.png" style="width: 678px; height: 109px;" />
 * Input: s = "7+3*1*2", answers = [20,13,42]
 * Output: 7
 * Explanation: As illustrated above, the correct answer of the expression is 13, therefore one student is rewarded 5 points: [20,<u>13</u>,42]
 * A student might have applied the operators in this wrong order: ((7+3)*1)*2 = 20. Therefore one student is rewarded 2 points: [<u>20</u>,13,42]
 * The points for the students are: [2,5,0]. The sum of the points is 2+5+0=7.
 *
 * Example 2:
 *
 * Input: s = "3+5*2", answers = [13,0,10,13,13,16,16]
 * Output: 19
 * Explanation: The correct answer of the expression is 13, therefore three students are rewarded 5 points each: [<u>13</u>,0,10,<u>13</u>,<u>13</u>,16,16]
 * A student might have applied the operators in this wrong order: ((3+5)*2 = 16. Therefore two students are rewarded 2 points: [13,0,10,13,13,<u>16</u>,<u>16</u>]
 * The points for the students are: [5,0,0,5,5,2,2]. The sum of the points is 5+0+0+5+5+2+2=19.
 *
 * Example 3:
 *
 * Input: s = "6+0*1", answers = [12,9,6,4,8,6]
 * Output: 10
 * Explanation: The correct answer of the expression is 6.
 * If a student had incorrectly done (6+0)*1, the answer would also be 6.
 * By the rules of grading, the students will still be rewarded 5 points (as they got the correct answer), not 2 points.
 * The points for the students are: [0,0,5,0,0,5]. The sum of the points is 10.
 *
 *  
 * Constraints:
 *
 * 	3 <= s.length <= 31
 * 	s represents a valid expression that contains only digits 0-9, '+', and '*' only.
 * 	All the integer operands in the expression are in the inclusive range [0, 9].
 * 	1 <= The count of all operators ('+' and '*') in the math expression <= 15
 * 	Test data are generated such that the correct answer of the expression is in the range of [0, 1000].
 * 	n == answers.length
 * 	1 <= n <= 10^4
 * 	0 <= answers[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-score-of-students-solving-math-expression/
// discuss: https://leetcode.com/problems/the-score-of-students-solving-math-expression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2019_example_1() {
        let s = "7+3*1*2".to_string();
        let answers = vec![20, 13, 42];

        let result = 7;

        assert_eq!(Solution::score_of_students(s, answers), result);
    }

    #[test]
    #[ignore]
    fn test_2019_example_2() {
        let s = "3+5*2".to_string();
        let answers = vec![13, 0, 10, 13, 13, 16, 16];

        let result = 19;

        assert_eq!(Solution::score_of_students(s, answers), result);
    }

    #[test]
    #[ignore]
    fn test_2019_example_3() {
        let s = "6+0*1".to_string();
        let answers = vec![12, 9, 6, 4, 8, 6];

        let result = 10;

        assert_eq!(Solution::score_of_students(s, answers), result);
    }
}
