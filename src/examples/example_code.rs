use std::io;

const THRESTHOLD: i32 = 1;
const LIFE: i32 = 3;

fn main() {
    const Time: i32 = 1000;
    let a = 1;
    let b = 2;
    let c = a + b;
    let d = c % 10;
    let e = 2 * a - b + 1;

    println!("{} {} {}", a, b, c);
}

fn get(x: i32) -> i32 {
    1
}

fn get_true(x: bool) -> bool {
    (x || (!x))
}

fn cube(x: i32) -> i32 {
    x * x * x
}

fn void_call() {
    let mut one = 1;
    assert_eq!(2, one);
}

fn get_name() -> String {
    String::from("Changhun Kim")
}

fn get_balance(x: Option<i32>) -> i32 {
    match x {
        None => { 0 },
        Some(balance) => { balance },
    }
}

fn is_zero(x : i32) -> bool { x == 0 }

fn x_and_y(x: i32, y: i32) -> i32 { x&y }

fn x_or_y(x: i32, y: i32) -> i32 { x|y }