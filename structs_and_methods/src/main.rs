struct Point {
    x: f64,
    y: f64,
}

struct Rect {
    top: f64,
    left: f64,
    width: f64,
    height: f64,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

enum Figure {
    Circle(Circle),
    Rect(Rect),
}

impl Rect {
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.left
            && p.y >= self.top
            && p.x <= self.left + self.width
            && p.y <= self.top + self.height
    }
}

impl Circle {
    fn contains(&self, p: &Point) -> bool {
        let dist = (self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y);
        dist <= self.radius * self.radius
    }
}

impl Figure {
    fn contains(&self, p: &Point) -> bool {
        match &self {
            Figure::Circle(circle) => circle.contains(p),
            Figure::Rect(rect) => rect.contains(p),
        }
    }
}

impl Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Figure {
    fn area(&self) -> f64 {
        match &self {
            Figure::Circle(circle) => circle.area(),
            Figure::Rect(rect) => rect.area(),
        }
    }
}

fn main() {
    let rect = Figure::Rect(Rect {
        left: 3.0,
        top: 9.1,
        width: 7.5,
        height: 3.2,
    });

    let circle = Figure::Circle(Circle {
        x: -1.0,
        y: 2.0,
        radius: 7.0,
    });

    let point = Point { x: 2.5, y: 6.0 };

    println!("Area of rect: {}", rect.area());
    println!("Area of circle: {}", circle.area());
    println!("Point in rect: {}", rect.contains(&point));
    println!("Point in circle: {}", circle.contains(&point));
}
