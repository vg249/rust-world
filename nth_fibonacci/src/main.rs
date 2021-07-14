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
             1 for dynamic");
    io::stdin()
        .read_line(&mut computation_type)
        .expect("Unable to get computation_type");


    let nth_fibonacci;

    if computation_type.trim() == "0" {
        nth_fibonacci = fibonacci(n);
    }
    else {
        nth_fibonacci = dyn_fibonacci(n);
    }

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
