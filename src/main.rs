enum Shape {
    Circle(f64),//variant with associated data (radius)
    Rectangle(f64, f64),
    Square(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    //pattern matching 
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

fn main() {
    println!("Hello, world!"); 
    let circle = Shape::Circle(2.0);
    let rectangle = Shape::Rectangle(2.0, 3.0);
    let square = Shape::Square(2.0);
    println!("Area of circle: {}", calculate_area(circle));
}

  