fn get_fitness_function(x: i32, y: i32) -> i32 {
    println!{"{}", x + y};

    let a = x + 1;
    return x + y / y - 2 * 3;

}

fn get_fitness_function2(x: i32, y: i32) {
    println!("{}", x + y);
}

fn main() {
    let test = vec!["one", "two", "two", "three"];
    let index = test.iter().position(|&r| r == "two").unwrap();
    println!("{}", index);
}

