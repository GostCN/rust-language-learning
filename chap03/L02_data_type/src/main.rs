use std::io;

fn main() {
    //error: type annotations needed
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of number is: {}", guess);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 取余
    let remainder = 43 % 5;
    println!("The value of quotient is: {} {}", quotient, remainder);

    let t = true;
    let f: bool = false; // 显式指定类型注解

    //char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //tup
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of one is: {}", one);


    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];         //let a = [3, 3, 3, 3, 3];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);

    //error: index out of bounds
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);

    //error: index out of bounds,if index greater 4;
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


}
