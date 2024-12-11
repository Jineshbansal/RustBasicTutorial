use chrono::{Local,Utc};
fn main() {
    let local = Local::now();
    let utc = Utc::now();
    println!("Local time: {}", local);
    println!("UTC time: {}", utc);
}
 
  