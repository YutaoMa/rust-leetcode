impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; temperatures.len()];

        // Solution 1: Jump list
        for index in (0..temperatures.len() - 1).rev() {
            let mut prev_index = index + 1;

            while ans[prev_index] > 0 && temperatures[index] >= temperatures[prev_index] {
                prev_index += ans[prev_index] as usize;
            }

            if temperatures[index] < temperatures[prev_index] {
                ans[index] = (prev_index - index) as i32;
            }
        }

        // Solution 2: Monotonic stack
        // let mut stack: Vec<usize> = Vec::new();
        // 
        // for (index, &temprature) in temperatures.iter().enumerate() {
        //     while let Some(&prev_index) = stack.last()  {
        //         if temprature <= temperatures[prev_index] {
        //             break;
        //         }

        //         stack.pop();
        //         ans[prev_index] = (index - prev_index) as i32;
        //     }
        //     
        //     stack.push(index);
        // }

        ans
    }
}
