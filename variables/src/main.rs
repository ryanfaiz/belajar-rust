fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    const SCORE: u32 = 100;

    println!("Your score is: {SCORE}");

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    println!("3 Hours in Seconds: {THREE_HOURS_IN_SECONDS}");


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";

    println!("The value of spaces is: {spaces}");

    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");

}
