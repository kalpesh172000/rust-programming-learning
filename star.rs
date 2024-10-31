use std::io::{ self, Write };
fn main() {
    let mut input_stream = String::new();
    print!("Enter a number: ");
    io::stdout().flush();
    std::io::stdin().read_line(&mut input_stream);
    let num: i32 = input_stream
        .trim()
        .parse()
        .expect("make sure you are inputing only number in range i32");

    for i in 0..num {
        for j in 0..(num-1-i){
            print!(" ");
        }
        for k in 0..i+1{
            print!("* ");
        }
        println!();
    }
}
