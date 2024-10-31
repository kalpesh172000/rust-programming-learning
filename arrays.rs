fn main()
{
    let mut nums:[u16; 5] = [4, 2, 3, 4, 5];
    let mut num:[u16; 5] = [0; 5];
    //this method is better that later 
    for i in num.iter_mut(){
        println!("{}", i); 
    }

    //this method is not as good as first one
    let ref mut nums:[u16; 5] = [4, 2, 3, 4, 5];
    for i in nums{
        *i = 12;
        println!("{}", i); 
    }
}