// Copyright 2024 by mut37581 on Discord.
// All Rights Reserved.
// Copy provided here for educational purposes only.

fn main() {
    //selecting the shape
    println!("Choose the correct shape you wish to Calcualate the area for:");
    println!("1. Square");
    println!("2. Rectangle");
    println!("3. Triangle");
    let mut shape = String::new();
    io::stdin().read_line(&mut shape).expect("Failed to read line");
    let shape: u32 = shape.trim().parse().expect("Pease enter a valid number i,e 1, 2 or 3");

    match shape {
        1=>{
            // prompt for square
            // as a squares length is the same as its width only need either one not both
            println!("please enter the length of the square");
            let mut side = String::new();
            io::stdin().read_line(&mut side).expect("Failed to read line");
            let side: f64 = side.trim().parse().expect("Please enter a valid number i,e 10");
            // calcuating the area of the square
            let area: f64 = side * side;
            println!("the dimensions of the square");
            println!("------------------------------------------------------");
            println!("the side length of the square: {:.2}", side);
            println!("the area of the square is :{:.2} square units", area);
            println!("-------------------------------------------------------");
        }
        
        2=> {
        // width prompt for rectangle
        println!("please enter the width of the rectangle");
        let mut width = String::new();
        io::stdin().read_line(&mut width).expect("Failed to read line");
        let width: f64 = width.trim().parse().expect("Please enter a vaild number i,e 10");
            
        // length prompt for rectangle 
        println!("Please enter the length of the rectangle");
        let mut length = String::new();
        io::stdin().read_line(&mut length).expect("failed to read line");
        let length: f64 = length.trim().parse().expect("Please enter a vaild number i.e 10");
        // calcuating the area 
        let area: f64 = width * length;
        println!("The dimensions of the rectangle are:");
        println!("---------------------------------------------------------");
        println!("Width: {:.2}", width);
        println!("length: {:.2}", length);
        println!("the area of the rectangle is {:.2} square units", area);
        println!("----------------------------------------------------------");
        
        }
        
        3 =>{
            // the base prompt for the triangle
            println!("please enter the base length of the triangle:");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("failed to read line");
            let base : f64 = base.trim().parse().expect("Please enter a vaild number i,e 10");

            // height promt for triangle
            println!("Please enter the height of the triangle");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read and line");
            let height: f64 = height.trim().parse().expect("Please enter a vaild number");
            
            //calcualting the area of the triangle

            let area = 0.5 * base * height;
            println!("the dimensions of the triangle:");
            println!("--------------------------------------------------------------");
            println!("Base: {:.2}", base);
            println!("Height: {:.2}", height);
            println!("the area of the triangle is: {:.2} sqaure units", area);
            println!("--------------------------------------------------------------");
            
        }
        _=>{
            println!("Invaild choice ! please only choose 1,2,3")
        }
    }
}