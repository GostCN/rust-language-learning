fn main() {
    /**

        //cannot assign twice to immutable variable `x`
        let x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);

    */

    /**

        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);

    */

    const MAX_POINTS: u32 = 100_000;

    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let x = 20;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
