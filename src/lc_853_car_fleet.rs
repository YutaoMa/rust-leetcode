use crate::Solution;

// @lc code=start
use std::collections::BTreeMap;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut map: BTreeMap<i32, f32> = BTreeMap::new();
        for (i, pos) in position.iter().enumerate() {
            // -pos as key to start with closest-to-target car
            map.insert(-pos, (target - pos) as f32 / speed[i] as f32);
        }

        // [car1: x min --(will catch up to car2 if x < y)--> car2: y min --(car2 is a new carfleet leader if y > z)--> car 3: z min]

        let mut ans: i32 = 0;
        let mut curr: f32 = 0.0;

        // BTreeMap guarantees val is sorted by key
        for (_, &val) in map.iter() {
            if val > curr {
                curr = val;
                ans += 1;
            }
        }

        ans
    }
}
// @lc code=end
