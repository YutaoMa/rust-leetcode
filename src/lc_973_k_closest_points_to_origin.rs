use crate::Solution;

// @lc code=start
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::iter::FromIterator;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Point> = BinaryHeap::new();

        for point in points.iter() {
            heap.push(point.into());
            if heap.len() == (k + 1) as usize {
                heap.pop();
            }
        }

        heap.iter().collect::<Vec<Vec<i32>>>()
    }
}

#[derive(Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn distance_to_origin_squared(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.distance_to_origin_squared().cmp(&other.distance_to_origin_squared())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Vec<i32>> for Point {
    fn from(point: &Vec<i32>) -> Self {
        Point { x: point[0], y: point[1] }
    }
}

impl From<&Point> for Vec<i32> {
    fn from(point: &Point) -> Self {
        vec![point.x, point.y]
    }
}

impl<'a> FromIterator<&'a Point> for Vec<Vec<i32>> {
    fn from_iter<T: IntoIterator<Item = &'a Point>>(iter: T) -> Self {
        let mut points: Vec<Vec<i32>> = Vec::new();
        for point in iter {
            points.push(point.into());
        }
        points
    }
}
// @lc code=end
