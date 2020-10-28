// Function types are explicit

// fn sqr(x: f64) -> f64 {
//     return x * x;
// }

// no semicolon = return
fn sqr(x: f64) -> f64 {
    x * x
}

fn main() {
    let res = sqr(2.0);
    println!("Square is {}", res);
}