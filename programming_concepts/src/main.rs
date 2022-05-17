use std::io;

fn main() {

    let x = 1;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner Scope: {}", x);
    }

    println!("Outer scope {}", x);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index!");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Unable to read line");


    let index: usize = index
        .trim()
        .parse()
        .expect("Please enter a numeric value");

    let value = a[index];

    println!("The value for the index {} is {}", index, value);



    

}
