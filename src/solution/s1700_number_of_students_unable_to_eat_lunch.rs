/**
 * [1700] Number of Students Unable to Eat Lunch
 *
 * The school cafeteria offers circular and square sandwiches at lunch break, referred to by numbers 0 and 1 respectively. All students stand in a queue. Each student either prefers square or circular sandwiches.
 * The number of sandwiches in the cafeteria is equal to the number of students. The sandwiches are placed in a stack. At each step:
 *
 * 	If the student at the front of the queue prefers the sandwich on the top of the stack, they will take it and leave the queue.
 * 	Otherwise, they will leave it and go to the queue's end.
 *
 * This continues until none of the queue students want to take the top sandwich and are thus unable to eat.
 * You are given two integer arrays students and sandwiches where sandwiches[i] is the type of the i^​​​​​​th sandwich in the stack (i = 0 is the top of the stack) and students[j] is the preference of the j^​​​​​​th student in the initial queue (j = 0 is the front of the queue). Return the number of students that are unable to eat.
 *  
 * Example 1:
 *
 * Input: students = [1,1,0,0], sandwiches = [0,1,0,1]
 * Output: 0
 * Explanation:
 * - Front student leaves the top sandwich and returns to the end of the line making students = [1,0,0,1].
 * - Front student leaves the top sandwich and returns to the end of the line making students = [0,0,1,1].
 * - Front student takes the top sandwich and leaves the line making students = [0,1,1] and sandwiches = [1,0,1].
 * - Front student leaves the top sandwich and returns to the end of the line making students = [1,1,0].
 * - Front student takes the top sandwich and leaves the line making students = [1,0] and sandwiches = [0,1].
 * - Front student leaves the top sandwich and returns to the end of the line making students = [0,1].
 * - Front student takes the top sandwich and leaves the line making students = [1] and sandwiches = [1].
 * - Front student takes the top sandwich and leaves the line making students = [] and sandwiches = [].
 * Hence all students are able to eat.
 *
 * Example 2:
 *
 * Input: students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= students.length, sandwiches.length <= 100
 * 	students.length == sandwiches.length
 * 	sandwiches[i] is 0 or 1.
 * 	students[i] is 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
// discuss: https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        (0..1000)
            .fold((0, students, sandwiches), |(n, mut st, mut sa), _| {
                if st.is_empty() || n == st.len() {
                    return (n, st, sa);
                }
                match st[0] == sa[0] {
                    true => {
                        st.remove(0);
                        sa.remove(0);
                        (0, st, sa)
                    }
                    false => {
                        st.push(st[0]);
                        st.remove(0);
                        (n + 1, st, sa)
                    }
                }
            })
            .1
            .len() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1700_example_1() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];

        let result = 0;

        assert_eq!(Solution::count_students(students, sandwiches), result);
    }

    #[test]
    fn test_1700_example_2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];

        let result = 3;

        assert_eq!(Solution::count_students(students, sandwiches), result);
    }
}
