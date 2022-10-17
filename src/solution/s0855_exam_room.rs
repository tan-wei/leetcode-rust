/**
 * [0855] Exam Room
 *
 * There is an exam room with n seats in a single row labeled from 0 to n - 1.
 * When a student enters the room, they must sit in the seat that maximizes the distance to the closest person. If there are multiple such seats, they sit in the seat with the lowest number. If no one is in the room, then the student sits at seat number 0.
 * Design a class that simulates the mentioned exam room.
 * Implement the ExamRoom class:
 *
 * 	ExamRoom(int n) Initializes the object of the exam room with the number of the seats n.
 * 	int seat() Returns the label of the seat at which the next student will set.
 * 	void leave(int p) Indicates that the student sitting at seat p will leave the room. It is guaranteed that there will be a student sitting at seat p.
 *
 *  
 * <strong class="example">Example 1:
 *
 * Input
 * ["ExamRoom", "seat", "seat", "seat", "seat", "leave", "seat"]
 * [[10], [], [], [], [], [4], []]
 * Output
 * [null, 0, 9, 4, 2, null, 5]
 * Explanation
 * ExamRoom examRoom = new ExamRoom(10);
 * examRoom.seat(); // return 0, no one is in the room, then the student sits at seat number 0.
 * examRoom.seat(); // return 9, the student sits at the last seat number 9.
 * examRoom.seat(); // return 4, the student sits at the last seat number 4.
 * examRoom.seat(); // return 2, the student sits at the last seat number 2.
 * examRoom.leave(4);
 * examRoom.seat(); // return 5, the student sits at the last seat number 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 * 	It is guaranteed that there is a student sitting at seat p.
 * 	At most 10^4 calls will be made to seat and leave.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/exam-room/
// discuss: https://leetcode.com/problems/exam-room/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct ExamRoom {
    memo: std::collections::BTreeSet<i32>,
    n: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    fn new(n: i32) -> Self {
        Self {
            n,
            memo: std::collections::BTreeSet::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        let len = self.memo.len();
        if len == 0 {
            self.memo.insert(0);
            0
        } else if len == 1 {
            let center = self.memo.iter().next().unwrap();
            let zerov = *center + 1;
            let maxv = self.n - center;

            if zerov < maxv {
                self.memo.insert(self.n - 1);
                self.n - 1
            } else {
                self.memo.insert(0);
                0
            }
        } else {
            let mut target_val = 0;
            let mut iter = self.memo.iter();
            let mut lv = iter.next().unwrap();
            let mut insert_val = 0;
            while let Some(cv) = iter.next() {
                let v = (cv - lv) / 2;

                if target_val < v {
                    insert_val = (cv + lv) / 2;
                    target_val = v;
                }
                lv = cv;
            }

            if !self.memo.contains(&0) {
                let v = (self.memo.iter().next().unwrap() + 1) / 2;
                if target_val <= v {
                    insert_val = v;
                    target_val = 0;
                }
            }

            if !self.memo.contains(&(self.n - 1)) {
                let v = (self.n - lv) / 2;
                if target_val < v {
                    insert_val = (self.n + lv) / 2;
                    target_val = self.n - 1;
                }
            }

            self.memo.insert(insert_val);
            insert_val as i32
        }
    }

    fn leave(&mut self, p: i32) {
        self.memo.remove(&p);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0855_example_1() {
        let mut exam_room = ExamRoom::new(10);
        assert_eq!(exam_room.seat(), 0); // return 0, no one is in the room, then the student sits at seat number 0.
        assert_eq!(exam_room.seat(), 9); // return 9, the student sits at the last seat number 9.
        assert_eq!(exam_room.seat(), 4); // return 4, the student sits at the last seat number 4.
        assert_eq!(exam_room.seat(), 2); // return 2, the student sits at the last seat number 2.
        exam_room.leave(4);
        assert_eq!(exam_room.seat(), 5); // return 5, the student sits at the last seat number 5.
    }
}
