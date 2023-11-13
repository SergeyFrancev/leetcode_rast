use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct SeatManager {
    // Use Reverse for - reverse order in queue
    unreserved: BinaryHeap<Reverse<i32>>,
    size: i32,
    busy: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        Self { unreserved: BinaryHeap::from([]), size: n, busy: 0 }
    }
    
    fn reserve(&mut self) -> i32 {
        if self.unreserved.len() > 0 {
            return self.unreserved.pop().unwrap().0;
        }
        if self.busy < self.size {
            self.busy += 1;
            return self.busy
        }
        return  -1
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.unreserved.push(Reverse(seat_number));
    }
}