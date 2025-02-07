
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left:i32 = 0;
    let mut right:i32 = height.len() as i32 - 1;
    let mut max_area = 0;
    while left < right {
        let temp_width = right - left;
        let temp_area = height[left as usize].min(height[right as usize]) * temp_width;
        if temp_area > max_area {
            max_area = temp_area;
        }
        if height[left as usize] < height[right as usize] {
            left += 1;
        } else {
            right -= 1;
        }
    } 
    max_area
}

fn main() {
    println!("Hello, world!");
}
