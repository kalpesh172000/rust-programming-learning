fn main() {
    let mut name = String::from("kalpesh");
    println!("before calling this function {}", name);
    who(& name); 
}

fn who(name: & String) {
    name.clear();
    name.push_str("samyak");
    println!("{:?} called this function", name);
}
