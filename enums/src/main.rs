// enum Direction {
//     North,
//     South,
//     East, 
//     West,
// }

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    // let current_direction = Direction::North;

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(10.0);
    let rectangle = Shape::Rectangle(5.0, 10.0);

    let circle_area = calculate_area(circle);
    let square_area = calculate_area(square);
    let rectangle_area = calculate_area(rectangle);

    println!("Circle Area: {}, Square Area: {}, Rectangle Area: {}", circle_area, square_area, rectangle_area);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}