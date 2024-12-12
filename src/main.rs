pub trait Summary {
    fn summarize(&self) -> String;
}

struct User{
    name: String,
    age: u32,
}

impl Summary for User { 
    fn summarize(&self) -> String {
        format!("Name: {}, Age: {}", self.name, self.age)
    }
} 
pub fn notify(item:&impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn main() { 
    let user = User {
        name: String::from("John"),
        age: 30,
    };
    notify(&user);

}

   