mod figures;

use figures::{Circle, Figure, Point, Rect};

fn main() {
    let rect = Figure::Rect(Rect {
        left: 3,
        top: 9,
        width: 7,
        height: 3,
    });

    let circle = Figure::Circle(Circle {
        x: -1,
        y: 2,
        radius: 7,
    });

    let point = Point { x: 2, y: 6 };

    println!("Area of rect: {}", rect.area());
    println!("Area of circle: {}", circle.area());
    println!("Point in rect: {}", rect.contains(&point));
    println!("Point in circle: {}", circle.contains(&point));
}
