fn main() {
    let fullname = " Tutorials Point \r\n";
    println!("Before trim ");
    println!("length is {} \"{}\"", fullname.len(), fullname);
    println!();
    println!("After trim ");
    println!("length is {} \"{}\"", fullname.trim().len(), fullname.trim());
}
