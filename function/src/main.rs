fn main() {
    println!("Hello, world!");

    let _m = another_function(-88);

    let x = {
        let mut y = 5;
        y = y + 5;
        y
    };
    let m  = another_function(x);

    println!("Output: {}", m);

}

fn another_function(x: i32) -> i32 {

    println!("Another function, Another value: {}", x);

    let y = x * 56;

    y
}
