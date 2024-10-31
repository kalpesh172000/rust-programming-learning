fn main()
{
    let tuple:(u8, i32, f64) = (1, 2, 3.4);
    display(tuple);
}

fn display(tuple:(u8,i32,f64)){
    let (a, b) = (tuple.0, tuple.1);
    let (p,q,r) = tuple;
    println!("a: {}, b: {}", a, b); 
    println!("p: {}, q: {}, r: {}", p, q, r);
}