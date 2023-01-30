fn main() {
    let x : f64 = 5.0;
    let x = x+1.0;
    // Shadowing
    {
        let x = x*3.0;
        println!("{x}")
    }
    println! ("{x}")
}