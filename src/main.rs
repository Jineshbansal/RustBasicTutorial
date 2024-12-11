struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn debug()->i32{
        5
    }
}

fn main() {
    println!("Hello, world!");
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("rect is {:?}", rect.area());
    println!("rect is {:?}", Rect::debug());
}

  