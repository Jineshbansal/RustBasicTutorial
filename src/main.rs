fn main() {
    println!("Hello, world!");
    println!("{}",is_even(2));

}
// General practice in rust to write in snake case
fn is_even(n:i32)->bool{
    if  n%2==0 {
        return true;
    }
    else{
        return false;
    }
}