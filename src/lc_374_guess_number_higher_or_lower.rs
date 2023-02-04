#![allow(unused_variables)]
use crate::Solution;

fn guess(n: i32) -> i32 {
    unimplemented!()
}

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    pub fn guess_number(n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if guess(mid) == 1 {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }
}
