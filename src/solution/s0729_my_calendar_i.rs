/**
 * [0729] My Calendar I
 *
 * You are implementing a program to use as your calendar. We can add a new event if adding the event will not cause a double booking.
 * A double booking happens when two events have some non-empty intersection (i.e., some moment is common to both events.).
 * The event can be represented as a pair of integers start and end that represents a booking on the half-open interval [start, end), the range of real numbers x such that start <= x < end.
 * Implement the MyCalendar class:
 *
 * 	MyCalendar() Initializes the calendar object.
 * 	boolean book(int start, int end) Returns true if the event can be added to the calendar successfully without causing a double booking. Otherwise, return false and do not add the event to the calendar.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MyCalendar", "book", "book", "book"]
 * [[], [10, 20], [15, 25], [20, 30]]
 * Output
 * [null, true, false, true]
 * Explanation
 * MyCalendar myCalendar = new MyCalendar();
 * myCalendar.book(10, 20); // return True
 * myCalendar.book(15, 25); // return False, It can not be booked because time 15 is already booked by another event.
 * myCalendar.book(20, 30); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
 *  
 * Constraints:
 *
 * 	0 <= start < end <= 10^9
 * 	At most 1000 calls will be made to book.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/my-calendar-i/
// discuss: https://leetcode.com/problems/my-calendar-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(Default)]
struct MyCalendar {
    events: std::collections::BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        self.events.iter().all(|&(s, e)| end <= s || start >= e) && self.events.insert((start, end))
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0729_example_1() {
        let mut my_calendar = MyCalendar::new();
        assert_eq!(my_calendar.book(10, 20), true);
        assert_eq!(my_calendar.book(15, 25), false);
        assert_eq!(my_calendar.book(20, 30), true);
    }
}
