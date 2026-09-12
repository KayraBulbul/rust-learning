fn main() {
    // Shadowing
    let spaces = "      ";
    let spaces = spaces.len();

    println!("spaces = {spaces}");

    let n = 10;
    {
        let n = 5;
        println!("Inner n = {n}"); // Should print 5
    }
    println!("Outer n = {n}"); // Should print 10

    // Explicit type declaration
    let number: i32 = 50;
    println!("number = {number}");

    // Cannot do `let num: f64 = 50;`
    let num: f64 = 50.0;
    println!("number = {num}");
}
