fn main() {

    // If condition
    let number = 3;

    if number < 5 {
        println!("Condition is true");
    }
    else {
        println!("false");
    }

    // Assign from if condition
    let condition = true;
    let s = if condition { 67 } else { 74 };

    println!("The Value of the number is: {}", s);


    // Loop
    let mut counter = 0;

    let m = loop {
        counter += 1;

        if counter == 7 {
            break counter + 9;
        }
    };
    
    println!("The Value of the counter is: {}", m);
    
    // Loop with match, break and continue
    counter = 0;

    let m = loop {
        counter += 1;
        match counter {
            7 => break counter + 9,
            _ => continue,
        }
    };
    
    println!("The Value of the counter is: {}", m);

    // Loop array with iter
    let m = [1, 2, 3, 4, 5, 6];

    for element in m.iter() {
        println!("The element is {}", element);
    }

    // Loop with Range
    for i in 4..7 {
        println!("The i is {}", i);
    }

    // Loop with Range Reverse
    for i in (4..7).rev() {
        println!("The i is {}", i);
    }

}
