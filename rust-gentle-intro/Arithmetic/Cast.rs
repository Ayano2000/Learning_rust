// cast values

fn main() {
    let mut sum = 0.0;
    // the compiler assumes i is an int (i32)
    for i in 0..5 {
        // i needs to be cast from an int to a float.
        // as f64 takes care of this.
        sum += i as f64;
    }
    println!("sum is {}", sum)
}