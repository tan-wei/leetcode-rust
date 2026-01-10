/**
 * [1146] Snapshot Array
 *
 * Implement a SnapshotArray that supports the following interface:
 *
 * 	SnapshotArray(int length) initializes an array-like data structure with the given length. Initially, each element equals 0.
 * 	void set(index, val) sets the element at the given index to be equal to val.
 * 	int snap() takes a snapshot of the array and returns the snap_id: the total number of times we called snap() minus 1.
 * 	int get(index, snap_id) returns the value at the given index, at the time we took the snapshot with the given snap_id
 *
 *  
 * Example 1:
 *
 * Input: ["SnapshotArray","set","snap","set","get"]
 * [[3],[0,5],[],[0,6],[0,0]]
 * Output: [null,null,0,null,5]
 * Explanation:
 * SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
 * snapshotArr.set(0,5);  // Set array[0] = 5
 * snapshotArr.snap();  // Take a snapshot, return snap_id = 0
 * snapshotArr.set(0,6);
 * snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5
 *  
 * Constraints:
 *
 * 	1 <= length <= 5 * 10^4
 * 	0 <= index < length
 * 	0 <= val <= 10^9
 * 	0 <= snap_id < (the total number of times we call snap())
 * 	At most 5 * 10^4 calls will be made to set, snap, and get.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/snapshot-array/
// discuss: https://leetcode.com/problems/snapshot-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct SnapshotArray {
    snapshots: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snapshots: vec![vec![]; length as usize],
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snapshots[index as usize].push((self.snap_id, val));
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let snaps: &Vec<(i32, i32)> = &self.snapshots[index as usize];

        let (mut l, mut r) = (-1, snaps.len() as i32 - 1);

        while l < r {
            let mid = l + (r - l + 1) / 2;

            if snaps[mid as usize].0 <= snap_id {
                l = mid;
            } else {
                r = mid - 1;
            }
        }

        if l >= 0 { snaps[l as usize].1 } else { 0 }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1146_example_1() {
        let mut snapshot_arr = SnapshotArray::new(3); // set the length to be 3
        snapshot_arr.set(0, 5); // Set array[0] = 5
        assert_eq!(snapshot_arr.snap(), 0); // Take a snapshot, return snap_id = 0
        snapshot_arr.set(0, 6);
        assert_eq!(snapshot_arr.get(0, 0), 5); // Get the value of array[0] with snap_id = 0, return 5
    }
}
