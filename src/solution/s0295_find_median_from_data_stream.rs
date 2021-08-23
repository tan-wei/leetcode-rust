/**
 * [295] Find Median from Data Stream
 *
 * The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.
 *
 * 	For example, for arr = [2,3,4], the median is 3.
 * 	For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
 *
 * Implement the MedianFinder class:
 *
 * 	MedianFinder() initializes the MedianFinder object.
 * 	void addNum(int num) adds the integer num from the data stream to the data structure.
 * 	double findMedian() returns the median of all elements so far. Answers within 10^-5 of the actual answer will be accepted.
 *
 *  
 * Example 1:
 *
 * Input
 * ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
 * [[], [1], [2], [], [3], []]
 * Output
 * [null, null, null, 1.5, null, 2.0]
 * Explanation
 * MedianFinder medianFinder = new MedianFinder();
 * medianFinder.addNum(1);    // arr = [1]
 * medianFinder.addNum(2);    // arr = [1, 2]
 * medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
 * medianFinder.addNum(3);    // arr[1, 2, 3]
 * medianFinder.findMedian(); // return 2.0
 *
 *  
 * Constraints:
 *
 * 	-10^5 <= num <= 10^5
 * 	There will be at least one element in the data structure before calling findMedian.
 * 	At most 5 * 10^4 calls will be made to addNum and findMedian.
 *
 *  
 * Follow up:
 *
 * 	If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
 * 	If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-median-from-data-stream/
// discuss: https://leetcode.com/problems/find-median-from-data-stream/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
#[derive(Default)]
struct MedianFinder {
    small: std::collections::BinaryHeap<i32>,
    large: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    odd: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        if self.odd {
            self.small.push(num);
            if let Some(max) = self.small.pop() {
                self.large.push(std::cmp::Reverse(max));
            }
        } else {
            self.large.push(std::cmp::Reverse(num));
            if let Some(std::cmp::Reverse(min)) = self.large.pop() {
                self.small.push(min);
            }
        }
        self.odd = !self.odd;
    }

    fn find_median(&self) -> f64 {
        if self.odd {
            *self.small.peek().unwrap() as f64
        } else {
            (self.small.peek().unwrap() + self.large.peek().unwrap().0) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0295_example_1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1); // arr = [1]
        median_finder.add_num(2); // arr = [1, 2]
        assert_eq!(median_finder.find_median(), 1.5); // return 1.5 (i.e., (1 + 2) / 2)
        median_finder.add_num(3); // arr[1, 2, 3]
        assert_eq!(median_finder.find_median(), 2.0); // return 2.0
    }
}
