use std::fs;
fn main() {
    println!("Hello, world!"); 
    let contents = fs::read_to_string("./Cargod. toml");
    match contents {
        Ok(c) => println!("{}", c),
        Err(e) => println!("Error: {}", e),
    }
}
 
  