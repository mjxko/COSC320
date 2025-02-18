#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let dataup = data.to_uppercase();

    println!("{dataup}");
}

fn main() {
    let data = "Rust is great!".to_string();
    let lastchar = get_char(&data);
    println!("our last char is {lastchar}");

    string_uppercase(&data);
}