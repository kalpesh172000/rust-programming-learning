fn main()
{
    let mut nums:[u16; 5] = [4, 2, 3, 4, 5];
    change(&mut nums[1..3]);
    for i in nums.iter(){
        println!("{}", i); 
    }
}

fn change(nums: &mut [u16]){
    nums[0] = 12;
}