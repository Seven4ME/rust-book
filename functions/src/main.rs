// Functions

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    call_me(3);

    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));

    let answer = square(3);
    println!("The square of 3 is {answer}");
}
