mod triangle;

use std::io::{stdin, stdout, Write};

use triangle::{
    calc_alfa_via_sinus_theorem, calc_beta_via_sinus_theorem, calc_c_side, calc_perimeter,
    calc_square, is_right_triangle,
};

fn read_leg(msg: std::string::String) -> f32 {
    print!("{}", msg);
    stdout().flush().unwrap();

    let mut leg = String::new();
    stdin().read_line(&mut leg).expect("Failed to read line");

    let leg = leg.trim().parse::<f32>().expect("A number expected!");

    leg
}

fn main() {
    let a = read_leg("a = ".to_string());
    let b = read_leg("b = ".to_string());

    let c = calc_c_side(a, b);
    if !is_right_triangle(a, b, c) {
        println!("Given a, b and c don't make a right triangle!");
        return;
    }

    let s = calc_square(a, b);
    let p = calc_perimeter(a, b, c);

    let alfa = calc_alfa_via_sinus_theorem(a, c);
    let beta = calc_beta_via_sinus_theorem(b, c);
    let gamma = 180.0 - alfa - beta;

    assert_eq!(alfa + beta + gamma, 180.0);

    println!("c = {}\n", c);
    println!("p = {}, s = {}", p, s);
    println!(
        "alfa = {:.2}, beta = {:.2}, gamma = {:.2}",
        alfa, beta, gamma
    );
}
