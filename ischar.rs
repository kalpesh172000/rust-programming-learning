fn main()
{
    let sent = String::from("1920 was like 3 time more that 12 % of the world");
    for x in sent.chars()
    {
        if (x>='a' && x<='z') || (x>= 'A' && x <= 'Z') || x = ' '
        {
            //println!("{} is a character", x);
        }
        else {
            println!("{} is not a character", x);
        }
    }
}