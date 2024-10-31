fn main() {
    let option = 4;
    let result = match option {
        0 => { 
            println!("code is 0"); 
            let pi:f32 = 1.1413;
            "pass"
        },
        1 => { 
            println!("code is 1"); "pass"
        },
        2 => { 
            println!("code is 2"); "pass"
        },
        _ => {
            println!("didn't match with anyone");
            let pi:f32 = 1.1413; "fail"
        }
    };
    println!("the values is : {}", result);
}
