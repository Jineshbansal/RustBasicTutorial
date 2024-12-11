fn main() {
    println!("Hello, world!");
    let s = String::from("JineshBansal");
    println!("The size of the string is: {}", get_string_size(&s));
}

fn get_string_size(s: &String)-> usize{
    s.chars().count()
}
 