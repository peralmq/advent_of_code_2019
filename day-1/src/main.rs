use std::io::{self, BufRead};

fn fuel(mass: i32) -> i32 {
    return (mass / 3i32) - 2i32;
}

fn fuel_fuel(mass: i32) -> i32 {
    let result = fuel(mass);
    if result < 0 {
        return 0;
    }

    return result + fuel_fuel(result);
}

fn main() {
    let mut result = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        result += fuel_fuel(mass);
        // println!("{}", fuel_fuel(mass));
    }
    println!("{}", result);
}
