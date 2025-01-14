/**
 * [1845] Seat Reservation Manager
 *
 * Design a system that manages the reservation state of n seats that are numbered from 1 to n.
 * Implement the SeatManager class:
 *
 * 	SeatManager(int n) Initializes a SeatManager object that will manage n seats numbered from 1 to n. All seats are initially available.
 * 	int reserve() Fetches the smallest-numbered unreserved seat, reserves it, and returns its number.
 * 	void unreserve(int seatNumber) Unreserves the seat with the given seatNumber.
 *
 *  
 * Example 1:
 *
 * Input
 * ["SeatManager", "reserve", "reserve", "unreserve", "reserve", "reserve", "reserve", "reserve", "unreserve"]
 * [[5], [], [], [2], [], [], [], [], [5]]
 * Output
 * [null, 1, 2, null, 2, 3, 4, 5, null]
 * Explanation
 * SeatManager seatManager = new SeatManager(5); // Initializes a SeatManager with 5 seats.
 * seatManager.reserve();    // All seats are available, so return the lowest numbered seat, which is 1.
 * seatManager.reserve();    // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
 * seatManager.unreserve(2); // Unreserve seat 2, so now the available seats are [2,3,4,5].
 * seatManager.reserve();    // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
 * seatManager.reserve();    // The available seats are [3,4,5], so return the lowest of them, which is 3.
 * seatManager.reserve();    // The available seats are [4,5], so return the lowest of them, which is 4.
 * seatManager.reserve();    // The only available seat is seat 5, so return 5.
 * seatManager.unreserve(5); // Unreserve seat 5, so now the available seats are [5].
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	1 <= seatNumber <= n
 * 	For each call to reserve, it is guaranteed that there will be at least one unreserved seat.
 * 	For each call to unreserve, it is guaranteed that seatNumber will be reserved.
 * 	At most 10^5 calls in total will be made to reserve and unreserve.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/seat-reservation-manager/
// discuss: https://leetcode.com/problems/seat-reservation-manager/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SeatManager {
    hp: std::collections::BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self {
            hp: std::collections::BinaryHeap::from((-n..=-1).collect::<Vec<i32>>()),
        }
    }

    fn reserve(&mut self) -> i32 {
        let i = self.hp.pop().unwrap();

        -i
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.hp.push(-seat_number);
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1845_example_1() {
        let mut seat_manager = SeatManager::new(5); // Initializes a SeatManager with 5 seats.

        assert_eq!(seat_manager.reserve(), 1); // All seats are available, so return the lowest numbered seat, which is 1.
        assert_eq!(seat_manager.reserve(), 2); // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
        seat_manager.unreserve(2); // Unreserve seat 2, so now the available seats are [2,3,4,5].
        assert_eq!(seat_manager.reserve(), 2); // The available seats are [2,3,4,5], so return the lowest of them, which is 2.
        assert_eq!(seat_manager.reserve(), 3); // The available seats are [3,4,5], so return the lowest of them, which is 3.
        assert_eq!(seat_manager.reserve(), 4); // The available seats are [4,5], so return the lowest of them, which is 4.
        assert_eq!(seat_manager.reserve(), 5); // The only available seat is seat 5, so return 5.
        seat_manager.unreserve(5); // Unreserve seat 5, so now the available seats are [5].
    }
}
