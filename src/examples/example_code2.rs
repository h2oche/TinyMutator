fn get_fitness_function(x: i32, y: i32) -> i32 {
    println!{"{}", x + y};
    x + y
}

fn get_fitness_function2(x: i32, y: i32) {
    println!("{}", x + y);
}

fn main() {
    get_fitness_function(1, 2);
    get_fitness_function2(1, 2);
}