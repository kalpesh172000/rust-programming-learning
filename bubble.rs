use std::io::Write;
fn main() {
    let mut nums: [u16; 10] = [4, 12, 7, 3, 5, 9, 6, 8, 2, 1];
    bubble(&mut nums);
    for i in nums.iter() {
        print!("{} ", i);
        let _ = std::io::stdout().flush();
    }
    println!();
}
fn bubble(nums: &mut [u16]) {
    let size = nums.len();

    for i in 0..size {
        for j in 0..size - i - 1 {
            if nums[j] > nums[j+1] {
                (nums[j], nums[j+1]) = (nums[j+1], nums[j]);
            }
        }
    }
}
