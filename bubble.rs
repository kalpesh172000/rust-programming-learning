

fn main()
{
    let mut nums:[u16; 10] = [4, 12, 7, 4, 5, 9, 7, 8, 2, 1]; 

    bubble_sort(&mut nums);
    for i in nums.iter(){
        println!("{}", i);
    }
}