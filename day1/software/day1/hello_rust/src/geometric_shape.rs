use std::fmt;

pub trait GeometricShape {
    fn calc_perimeter(&self) -> f64;
}

pub fn calc_perimeter(shape: &dyn GeometricShape) -> f64 {
    shape.calc_perimeter()
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
}

pub struct Triangle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GeometricShape for Circle {
    fn calc_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl GeometricShape for Rectangle {
    fn calc_perimeter(&self) -> f64 {
        2.0 * (self.x + self.y)
    }
}

impl GeometricShape for Triangle {
    fn calc_perimeter(&self) -> f64 {
        self.x + self.y + self.z
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!(f, "Circle: {{ radius = {} }}", self.radius)
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!(f, "Rectangle: {{ X = {}, Y = {} }}", self.x, self.y)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!(f, "Triangle: {{ X = {}, Y = {}, Z = {} }}", self.x, self.y, self.z)
    }
}