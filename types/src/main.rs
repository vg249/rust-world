use std::io;

fn main() {

    let sum = 5 + 4;
    println!("{}", sum);

    let difference = 56 - 45;
    println!("{}", difference);

    let product = 8 * 7;
    println!("{}", product);

    let quotient = 45.56 / 7.2;
    println!("{}", quotient);

    let remainder = 43 % 4;
    println!("{}", remainder);


    let x: (i32, f64, u8) = (-7, 45.43, 234);

    let _minus_seven = x.0;

    let _fortyfive_point_fortythree = x.1;

    let _twothrityfouur = x.2;

    let _a: [char; 6] = ['q', 'w', 'e', 'r', 't', 'y'];

    println!("{}", _a[0]);
    
    println!("Enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Unable to read the input index");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _a[index];

    println!("The value of the element in index {} is {}",
             index,
             element);
}
