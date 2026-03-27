/**
 * [2409] Count Days Spent Together
 *
 * Alice and Bob are traveling to Rome for separate business meetings.
 * You are given 4 strings arriveAlice, leaveAlice, arriveBob, and leaveBob. Alice will be in the city from the dates arriveAlice to leaveAlice (inclusive), while Bob will be in the city from the dates arriveBob to leaveBob (inclusive). Each will be a 5-character string in the format "MM-DD", corresponding to the month and day of the date.
 * Return the total number of days that Alice and Bob are in Rome together.
 * You can assume that all dates occur in the same calendar year, which is not a leap year. Note that the number of days per month can be represented as: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31].
 *  
 * Example 1:
 *
 * Input: arriveAlice = "08-15", leaveAlice = "08-18", arriveBob = "08-16", leaveBob = "08-19"
 * Output: 3
 * Explanation: Alice will be in Rome from August 15 to August 18. Bob will be in Rome from August 16 to August 19. They are both in Rome together on August 16th, 17th, and 18th, so the answer is 3.
 *
 * Example 2:
 *
 * Input: arriveAlice = "10-01", leaveAlice = "10-31", arriveBob = "11-01", leaveBob = "12-31"
 * Output: 0
 * Explanation: There is no day when Alice and Bob are in Rome together, so we return 0.
 *
 *  
 * Constraints:
 *
 * 	All dates are provided in the format "MM-DD".
 * 	Alice and Bob's arrival dates are earlier than or equal to their leaving dates.
 * 	The given dates are valid dates of a non-leap year.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-days-spent-together/
// discuss: https://leetcode.com/problems/count-days-spent-together/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let parse_num_date = |s: String| -> i32 {
            let mut it = s.split('-').map(|mds| mds.parse::<i32>().unwrap());
            let (month, day) = (it.next().unwrap(), it.next().unwrap());
            MONTHS[..(month - 1) as usize].iter().sum::<i32>() + day
        };

        let arrive_alice_day = parse_num_date(arrive_alice);
        let leave_alice_day = parse_num_date(leave_alice);
        let arrive_bob_day = parse_num_date(arrive_bob);
        let leave_bob_day = parse_num_date(leave_bob);

        let min_day = arrive_alice_day.max(arrive_bob_day);
        let max_day = leave_alice_day.min(leave_bob_day);
        0.max(max_day - min_day + 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2409_example_1() {
        let arrive_alice = "08-15".to_string();
        let leave_alice = "08-18".to_string();
        let arrive_bob = "08-16".to_string();
        let leave_bob = "08-19".to_string();

        let result = 3;

        assert_eq!(
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            result
        );
    }

    #[test]
    fn test_2409_example_2() {
        let arrive_alice = "10-01".to_string();
        let leave_alice = "10-31".to_string();
        let arrive_bob = "11-01".to_string();
        let leave_bob = "12-31".to_string();

        let result = 0;

        assert_eq!(
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob),
            result
        );
    }
}
