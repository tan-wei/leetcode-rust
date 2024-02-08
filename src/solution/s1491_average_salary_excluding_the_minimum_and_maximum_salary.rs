/**
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 *
 * You are given an array of unique integers salary where salary[i] is the salary of the i^th employee.
 * Return the average salary of employees excluding the minimum and maximum salary. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 *
 * Input: salary = [4000,3000,1000,2000]
 * Output: 2500.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 4000 respectively.
 * Average salary excluding minimum and maximum salary is (2000+3000) / 2 = 2500
 *
 * Example 2:
 *
 * Input: salary = [1000,2000,3000]
 * Output: 2000.00000
 * Explanation: Minimum salary and maximum salary are 1000 and 3000 respectively.
 * Average salary excluding minimum and maximum salary is (2000) / 1 = 2000
 *
 *  
 * Constraints:
 *
 * 	3 <= salary.length <= 100
 * 	1000 <= salary[i] <= 10^6
 * 	All the integers of salary are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/
// discuss: https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut salary = salary;
        salary.sort_unstable();
        salary
            .iter()
            .skip(1)
            .take(salary.len() - 2)
            .map(|x| *x as f64)
            .sum::<f64>()
            / (salary.len() - 2) as f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1491_example_1() {
        let salary = vec![4000, 3000, 1000, 2000];

        let result = 2500.00000;

        assert_f64_near!(Solution::average(salary), result);
    }

    #[test]
    fn test_1491_example_2() {
        let salary = vec![1000, 2000, 3000];

        let result = 2000.00000;

        assert_f64_near!(Solution::average(salary), result);
    }
}
