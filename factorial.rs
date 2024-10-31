use std::io::{self, Write};
fn main()
{
    print!("Enter a number: ");
    std::io::stdout().flush();
    let mut input_stream = String::new();
    io::stdin().read_line(&mut input_stream);
    let num:u16 = input_stream.trim().parse().expect("make sure you are inputing only number in range u8");
    println!("factorial of {} is {}", num, factorial(num));
}


fn factorial(n:u16)->u16{
    if n == 0 {
        return 1;
    }
    n * factorial(n-1)
}