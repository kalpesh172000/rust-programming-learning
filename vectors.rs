fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    //better cause no bound checking 
    for i in v.iter_mut() {
        *i = 12;
        println!("{}", i);
    }
    //little less efficient due to bound checking 
    for i in 0..v.len() {
        v[i] = 4;
        println!("{}", v[i]);
    }
}
