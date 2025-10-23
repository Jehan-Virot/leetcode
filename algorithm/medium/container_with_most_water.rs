use std::cmp;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = height.len() as i32 -1;
        while left != right{
            let value = *cmp::min(&height[left as usize], &height[right as usize]) as i32 * (right - left);
            max = if max < value {value} else {max};
            if height[left as usize] > height[right as usize]{
                right -= 1;
            } else {
                left += 1;
            }
        }
        return max
    }
}