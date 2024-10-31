use std::collections::HashMap;

fn main()
{
    let mut hm:HashMap<&str, u16> = HashMap::new();
    hm.insert("one", 1);
    hm.insert("two", 1);
    hm.insert("two", 2);

    println!("{}", hm.get(&"two").unwrap());
}