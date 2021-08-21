use std::io;

fn main() {

    println!("Nth Finbonaci!");

    println!("Enter the n:");

    let mut n = String::new();
    let mut computation_type = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Unable to get n");
   
    let n: u32 = n
        .trim()
        .parse()
        .expect("Invalid n value");

    println!("Enter the computation type:\n\
             0 for recursive\n\
             1 for dynamic\n\
             2 for dynamic without array");
    
    io::stdin()
        .read_line(&mut computation_type)
        .expect("Unable to get computation_type");
    
    let computation_type: u32 = computation_type
        .trim()
        .parse()
        .expect("Invalid computation type");

    let nth_fibonacci = match computation_type {
        0 => fibonacci(n),
        1 => dyn_fibonacci(n),
        2 => dyn_fibonacci_no_array(n),
        _ => 0,
    };

    println!("Nth Fibonacci: {}", nth_fibonacci);

}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    }
    else if n == 1 {
        1
    }
    else {
        let nth_fibonacci = fibonacci(n - 1) + fibonacci(n - 2);
        nth_fibonacci
    }
}

// Dynamic Programming
fn dyn_fibonacci(n: u32) -> u32 {
    let mut fibonacci_nums = vec![0; (n+2) as usize];
    
    fibonacci_nums[0] = 0;
    fibonacci_nums[1] = 1;

    for i in 2..(n+1) {
        fibonacci_nums[i as usize] = fibonacci_nums[(i-1) as usize] 
            + fibonacci_nums[(i-2) as usize];
    }
    
    fibonacci_nums[n as usize]
}

fn dyn_fibonacci_no_array(n: u32) -> u32 {

    let mut one_before = 0;
    let mut one_before_one_before = 1;

    let mut nth_fibonacci = 0;

    for _ in 1..(n+1) {
        nth_fibonacci = one_before + one_before_one_before;
        one_before_one_before = one_before;
        one_before = nth_fibonacci;
    }
    
    nth_fibonacci
}

