fn find_first_a(s:String)->Option<i32>{
    for c in s.chars().enumerate(){
        if c.1=='a'{
            return Some(c.0 as i32);
        }
    }
    return None;
}

fn main() {
    println!("Hello, world!"); 
    let indx=find_first_a(String::from("helloa"));
    match indx{
        Some(i)=>println!("Found at index:{}",i),
        None=>println!("Not found")
    } 


}

  