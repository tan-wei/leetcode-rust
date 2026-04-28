/**
 * [2446] Determine if Two Events Have Conflict
 *
 * You are given two arrays of strings that represent two inclusive events that happened on the same day, event1 and event2, where:
 *
 * 	event1 = [startTime1, endTime1] and
 * 	event2 = [startTime2, endTime2].
 *
 * Event times are valid 24 hours format in the form of HH:MM.
 * A conflict happens when two events have some non-empty intersection (i.e., some moment is common to both events).
 * Return true if there is a conflict between two events. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: event1 = ["01:15","02:00"], event2 = ["02:00","03:00"]
 * Output: true
 * Explanation: The two events intersect at time 2:00.
 *
 * Example 2:
 *
 * Input: event1 = ["01:00","02:00"], event2 = ["01:20","03:00"]
 * Output: true
 * Explanation: The two events intersect starting from 01:20 to 02:00.
 *
 * Example 3:
 *
 * Input: event1 = ["10:00","11:00"], event2 = ["14:00","15:00"]
 * Output: false
 * Explanation: The two events do not intersect.
 *
 *  
 * Constraints:
 *
 * 	event1.length == event2.length == 2
 * 	event1[i].length == event2[i].length == 5
 * 	startTime1 <= endTime1
 * 	startTime2 <= endTime2
 * 	All the event times follow the HH:MM format.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-if-two-events-have-conflict/
// discuss: https://leetcode.com/problems/determine-if-two-events-have-conflict/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        event1[0] <= event2[1] && event1[1] >= event2[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2446_example_1() {
        let event1 = vec_string!["01:15", "02:00"];
        let event2 = vec_string!["02:00", "03:00"];

        let result = true;

        assert_eq!(Solution::have_conflict(event1, event2), result);
    }

    #[test]
    fn test_2446_example_2() {
        let event1 = vec_string!["01:00", "02:00"];
        let event2 = vec_string!["01:20", "03:00"];

        let result = true;

        assert_eq!(Solution::have_conflict(event1, event2), result);
    }

    #[test]
    fn test_2446_example_3() {
        let event1 = vec_string!["10:00", "11:00"];
        let event2 = vec_string!["14:00", "15:00"];

        let result = false;

        assert_eq!(Solution::have_conflict(event1, event2), result);
    }
}
