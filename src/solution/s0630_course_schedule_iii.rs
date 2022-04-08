/**
 * [0630] Course Schedule III
 *
 * There are n different online courses numbered from 1 to n. You are given an array courses where courses[i] = [durationi, lastDayi] indicate that the i^th course should be taken continuously for durationi days and must be finished before or on lastDayi.
 * You will start on the 1^st day and you cannot take two or more courses simultaneously.
 * Return the maximum number of courses that you can take.
 *  
 * Example 1:
 *
 * Input: courses = [[100,200],[200,1300],[1000,1250],[2000,3200]]
 * Output: 3
 * Explanation:
 * There are totally 4 courses, but you can take 3 courses at most:
 * First, take the 1^st course, it costs 100 days so you will finish it on the 100^th day, and ready to take the next course on the 101^st day.
 * Second, take the 3^rd course, it costs 1000 days so you will finish it on the 1100^th day, and ready to take the next course on the 1101^st day.
 * Third, take the 2^nd course, it costs 200 days so you will finish it on the 1300^th day.
 * The 4^th course cannot be taken now, since you will finish it on the 3300^th day, which exceeds the closed date.
 *
 * Example 2:
 *
 * Input: courses = [[1,2]]
 * Output: 1
 *
 * Example 3:
 *
 * Input: courses = [[3,2],[4,3]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= courses.length <= 10^4
 * 	1 <= durationi, lastDayi <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-iii/
// discuss: https://leetcode.com/problems/course-schedule-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/course-schedule-iii/discuss/1187347/Rust-BinaryHeap-solution
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by_cached_key(|v| v[1]);
        let mut bh = std::collections::BinaryHeap::new();
        let mut total = 0;
        for course in &courses {
            bh.push(course[0]);
            total += course[0];
            if total > course[1] {
                if let Some(max) = bh.pop() {
                    total -= max;
                }
            }
        }
        bh.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0630_example_1() {
        let courses = vec![
            vec![100, 200],
            vec![200, 1300],
            vec![1000, 1250],
            vec![2000, 3200],
        ];
        let result = 3;

        assert_eq!(Solution::schedule_course(courses), result);
    }

    #[test]
    fn test_0630_example_2() {
        let courses = vec![vec![1, 2]];
        let result = 1;

        assert_eq!(Solution::schedule_course(courses), result);
    }

    #[test]
    fn test_0630_example_3() {
        let courses = vec![vec![3, 2], vec![4, 3]];
        let result = 0;

        assert_eq!(Solution::schedule_course(courses), result);
    }
}
