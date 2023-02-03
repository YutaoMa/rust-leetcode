impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let carry_over = false;        

        for digit in digits.iter_mut().rev() {
            match *digit == 9 {
                false => {
                    *digit += 1;
                    return digits;
                },
                true => {
                    *digit = 0;
                }
            }
        }

        digits.insert(0, 1);
        digits
    }
}
