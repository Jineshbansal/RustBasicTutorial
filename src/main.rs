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

fn main() {
    /*
    A trait defines the functionality particular type has and share with other types.
    We can also define default behavior for traits.
    */
    let user = User {
        name: String::from("John"),
        age: 25,
    };
    println!("{}", user.summarize()); 

}

   