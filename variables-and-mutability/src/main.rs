use std::io;

//variables
fn rustling1() {
    let x: u8 = 5;

    println!("x has the value {x}");
}

fn rustling2() {
    println!("Please type a number");
    let mut input_str: String = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    let x: i32 = input_str.trim().parse().expect("Please type a number!");

    if x == 10 {
        println!("x is 10");
    } else {
        println!("x is not 10");
    }
}

fn rustling3() {
    let x: i32 = 67;

    println!("Number {x}");
}

fn rustling4() {
    let mut x = 3;
    println!("number {x}");

    x = 5;
    println!("Number {x}");
}

fn rustling5() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number: i32 = 3;
    println!("Number plus two is: {}", number + 2);
}
const NUMBER: u32 = 3;

fn rustling6() {
    println!("Number: {NUMBER}");
}

fn main() {
    rustling1();
    rustling2();
    rustling3();
    rustling4();
    rustling5();
    rustling6();
}
