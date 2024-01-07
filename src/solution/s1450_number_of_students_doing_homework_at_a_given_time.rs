/**
 * [1450] Number of Students Doing Homework at a Given Time
 *
 * Given two integer arrays startTime and endTime and given an integer queryTime.
 * The ith student started doing their homework at the time startTime[i] and finished it at time endTime[i].
 * Return the number of students doing their homework at time queryTime. More formally, return the number of students where queryTime lays in the interval [startTime[i], endTime[i]] inclusive.
 *  
 * Example 1:
 *
 * Input: startTime = [1,2,3], endTime = [3,2,7], queryTime = 4
 * Output: 1
 * Explanation: We have 3 students where:
 * The first student started doing homework at time 1 and finished at time 3 and wasn't doing anything at time 4.
 * The second student started doing homework at time 2 and finished at time 2 and also wasn't doing anything at time 4.
 * The third student started doing homework at time 3 and finished at time 7 and was the only student doing homework at time 4.
 *
 * Example 2:
 *
 * Input: startTime = [4], endTime = [4], queryTime = 4
 * Output: 1
 * Explanation: The only student was doing their homework at the queryTime.
 *
 *  
 * Constraints:
 *
 * 	startTime.length == endTime.length
 * 	1 <= startTime.length <= 100
 * 	1 <= startTime[i] <= endTime[i] <= 1000
 * 	1 <= queryTime <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/
// discuss: https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time
            .iter()
            .zip(end_time.iter())
            .map(|(&start_time, &end_time)| query_time >= start_time && query_time <= end_time)
            .filter(|&is_in_interval| is_in_interval)
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1450_example_1() {
        let start_time = vec![1, 2, 3];
        let end_time = vec![3, 2, 7];
        let query_time = 4;

        let result = 1;

        assert_eq!(
            Solution::busy_student(start_time, end_time, query_time),
            result
        );
    }

    #[test]
    fn test_1450_example_2() {
        let start_time = vec![4];
        let end_time = vec![4];
        let query_time = 4;

        let result = 1;

        assert_eq!(
            Solution::busy_student(start_time, end_time, query_time),
            result
        );
    }
}
