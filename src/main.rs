// Copyright 2024, David Groves.
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Shape {
    Square,
    Rectangle,
    Triangle,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Shape::Square),
            "2" => Ok(Shape::Rectangle),
            "3" => Ok(Shape::Triangle),
            _ => Err(()),
        }
    }
}

struct Polygon {
    shape: Shape,
    height: f64,
    width: f64,
}

impl Polygon {
    fn area(&self) -> f64 {
        match self.shape {
            Shape::Square => {
                return self.height * self.width;
            }
            Shape::Rectangle => {
                return self.height * self.width;
            }
            Shape::Triangle => {
                return (self.height * self.width) / 2.0;
            }
        }
    }
    fn perimeter(&self) -> Option<f64> {
        match self.shape {
            Shape::Square => {
                return Some(4.0 * self.height);
            }
            Shape::Rectangle => {
                return Some(2.0 * (self.height + self.width));
            }
            Shape::Triangle => {
                return None;
            }
        }
    }
}

fn request_shape_from_user() -> Shape {
    println!("Choose the correct shape you wish to calcualate the area for:");
    println!("1. Square");
    println!("2. Rectangle");
    println!("3. Triangle");

    return input::<Shape>().get();
}

fn request_question_from_user(question: &str) -> f64 {
    println!("Please enter the {} of the shape", question);
    return input::<f64>().get();
}

fn get_user_input() -> Polygon {
    loop {
        let shape = request_shape_from_user();
        match shape {
            Shape::Square => {
                let side = request_question_from_user("length of a side");
                return Polygon {
                    shape: shape,
                    height: side,
                    width: side,
                };
            }
            Shape::Rectangle => {
                let width = request_question_from_user("width");
                let length = request_question_from_user("length");
                return Polygon {
                    shape: shape,
                    height: length,
                    width: width,
                };
            }
            Shape::Triangle => {
                let width = request_question_from_user("base length");
                let height = request_question_from_user("height");
                return Polygon {
                    shape: shape,
                    height: height,
                    width: width,
                };
            }
        }
    }
}

fn main() {
    let polygon = get_user_input();
    println!("");
    println!("");
    println!("This shape has the following properties:");
    println!("---------------------------------------------------------");
    println!("Shape: {:?}", polygon.shape);
    println!("Height: {:.2}", polygon.height);
    println!("Width: {:.2}", polygon.width);
    println!("----------------------------------------------------------");
    println!(
        "The area of the shape is: {:.2} square units",
        polygon.area()
    );
    match polygon.perimeter() {
        Some(perimeter) => {
            println!("The perimeter of the shape is: {:.2} units", perimeter);
        }
        None => {
            println!("The perimeter of the shape is not known");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_square() {
        let polygon = Polygon {
            shape: Shape::Square,
            height: 10.0,
            width: 10.0,
        };
        assert_eq!(polygon.area(), 100.0);
    }

    #[test]
    fn test_area_rectangle() {
        let polygon = Polygon {
            shape: Shape::Rectangle,
            height: 5.0,
            width: 5.0,
        };
        assert_eq!(polygon.area(), 25.0);
    }

    #[test]
    fn test_area_triangle() {
        let polygon = Polygon {
            shape: Shape::Triangle,
            height: 5.0,
            width: 5.0,
        };
        assert_eq!(polygon.area(), 12.5);
    }

    #[test]
    fn test_perimeter_square() {
        let polygon = Polygon {
            shape: Shape::Square,
            height: 10.0,
            width: 10.0,
        };
        assert_eq!(polygon.perimeter(), Some(40.0));
    }

    #[test]
    fn test_perimeter_rectangle() {
        let polygon = Polygon {
            shape: Shape::Rectangle,
            height: 5.0,
            width: 5.0,
        };
        assert_eq!(polygon.perimeter(), Some(20.0));
    }

    #[test]
    fn test_perimeter_triangle() {
        let polygon = Polygon {
            shape: Shape::Triangle,
            height: 5.0,
            width: 5.0,
        };
        assert_eq!(polygon.perimeter(), None);
    }
}
