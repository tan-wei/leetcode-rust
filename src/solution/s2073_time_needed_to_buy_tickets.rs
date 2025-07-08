/**
 * [2073] Time Needed to Buy Tickets
 *
 * There are n people in a line queuing to buy tickets, where the 0^th person is at the front of the line and the (n - 1)^th person is at the back of the line.
 * You are given a 0-indexed integer array tickets of length n where the number of tickets that the i^th person would like to buy is tickets[i].
 * Each person takes exactly 1 second to buy a ticket. A person can only buy 1 ticket at a time and has to go back to the end of the line (which happens instantaneously) in order to buy more tickets. If a person does not have any tickets left to buy, the person will leave the line.
 * Return the time taken for the person initially at position k (0-indexed) to finish buying tickets.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">tickets = [2,3,2], k = 2</span>
 * Output: <span class="example-io">6</span>
 * Explanation:
 *
 * 	The queue starts as [2,3,<u>2</u>], where the kth person is underlined.
 * 	After the person at the front has bought a ticket, the queue becomes [3,<u>2</u>,1] at 1 second.
 * 	Continuing this process, the queue becomes [<u>2</u>,1,2] at 2 seconds.
 * 	Continuing this process, the queue becomes [1,2,<u>1</u>] at 3 seconds.
 * 	Continuing this process, the queue becomes [2,<u>1</u>] at 4 seconds. Note: the person at the front left the queue.
 * 	Continuing this process, the queue becomes [<u>1</u>,1] at 5 seconds.
 * 	Continuing this process, the queue becomes [1] at 6 seconds. The kth person has bought all their tickets, so return 6.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">tickets = [5,1,1,1], k = 0</span>
 * Output: <span class="example-io">8</span>
 * Explanation:
 *
 * 	The queue starts as [<u>5</u>,1,1,1], where the kth person is underlined.
 * 	After the person at the front has bought a ticket, the queue becomes [1,1,1,<u>4</u>] at 1 second.
 * 	Continuing this process for 3 seconds, the queue becomes [<u>4]</u> at 4 seconds.
 * 	Continuing this process for 4 seconds, the queue becomes [] at 8 seconds. The kth person has bought all their tickets, so return 8.
 * </div>
 *  
 * Constraints:
 *
 * 	n == tickets.length
 * 	1 <= n <= 100
 * 	1 <= tickets[i] <= 100
 * 	0 <= k < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/time-needed-to-buy-tickets/
// discuss: https://leetcode.com/problems/time-needed-to-buy-tickets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        tickets.iter().enumerate().fold(0, |s, (i, &v)| {
            s + std::cmp::min(tickets[k as usize] - i32::from(i as i32 > k), v)
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2073_example_1() {
        let tickets = vec![2, 3, 2];
        let k = 2;

        let result = 6;

        assert_eq!(Solution::time_required_to_buy(tickets, k), result);
    }

    #[test]
    fn test_2073_example_2() {
        let tickets = vec![5, 1, 1, 1];
        let k = 0;

        let result = 8;

        assert_eq!(Solution::time_required_to_buy(tickets, k), result);
    }
}
