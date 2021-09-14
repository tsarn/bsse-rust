use num_traits::Num;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct Point<T: Num = f64> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rect<T: Num = f64> {
    pub top: T,
    pub left: T,
    pub width: T,
    pub height: T,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Circle<T: Num = f64> {
    pub x: T,
    pub y: T,
    pub radius: T,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Figure<T: Num = f64> {
    Circle(Circle<T>),
    Rect(Rect<T>),
}

impl<T: Num> Default for Point<T> {
    fn default() -> Self {
        Point {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T: Num> Default for Rect<T> {
    fn default() -> Self {
        Rect {
            top: T::zero(),
            left: T::zero(),
            width: T::one(),
            height: T::one(),
        }
    }
}

impl<T: Num> Default for Circle<T> {
    fn default() -> Self {
        Circle {
            x: T::zero(),
            y: T::zero(),
            radius: T::one(),
        }
    }
}

impl<T: Num + Hash> Hash for Point<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Num + Hash> Hash for Rect<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.top.hash(state);
        self.left.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<T: Num + Hash> Hash for Circle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.radius.hash(state);
    }
}

impl<T: Num + PartialOrd + Copy> Rect<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        p.x >= self.left
            && p.y >= self.top
            && p.x <= self.left + self.width
            && p.y <= self.top + self.height
    }
}

impl<T: Num + PartialOrd + Copy> Circle<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        let dist = (self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y);
        dist <= self.radius * self.radius
    }
}

impl<T: Num + PartialOrd + Copy> Figure<T> {
    pub fn contains(&self, p: &Point<T>) -> bool {
        match &self {
            Figure::Circle(circle) => circle.contains(p),
            Figure::Rect(rect) => rect.contains(p),
        }
    }
}

impl<T: Num + Copy> Rect<T> {
    pub fn area(&self) -> T {
        self.width * self.height
    }
}

impl<T: Num + Copy + Into<f64>> Circle<T> {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.into() * self.radius.into()
    }
}

impl<T: Num + Copy + Into<f64>> Figure<T> {
    pub fn area(&self) -> f64 {
        match &self {
            Figure::Circle(circle) => circle.area(),
            Figure::Rect(rect) => rect.area().into(),
        }
    }
}
