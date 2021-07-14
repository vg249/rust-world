use std::io;

fn main() {

    println!("Fahreinheit -> Celcius converter");
    println!("F: ");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to get fahrenheit");

    let fahrenheit: f64 = fahrenheit
        .trim()
        .parse()
        .expect("Invalid fahrenheit value");

    let celcius = (fahrenheit - 32.0) * (5.0/9.0);

    println!("C: {:.4}", celcius);
}
