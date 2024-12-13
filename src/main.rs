// Shorter lifetime is what the return type will have


fn longer_str<'a>(s1:&'a str, s2:&'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    } 
}


fn main() { 
    let longest_str;
    let s1 = String::from("small");
    {
        let s2 = String::from("longer");
        longest_str = longer_str(&s1, &s2);
    }
    println!("The longest string is: {}", longest_str); 
    
    


}

   