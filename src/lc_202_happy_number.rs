use crate::Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
       let digit_square_sum = |mut n: i32| -> i32 {
           let mut sum = 0;

           while n > 0 {
               let digit = n % 10;
               sum += digit * digit;
               n /= 10;
           }

           sum
       };

       let mut slow = n;
       let mut fast = n;

       while {
           slow = digit_square_sum(slow);
           fast = digit_square_sum(digit_square_sum(fast));
           slow != fast
       } {}

       slow == 1
    }
}
