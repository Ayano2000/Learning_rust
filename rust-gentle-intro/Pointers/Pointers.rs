// function takes a reference to the variable you want it to change
fn modifies(x: &mut f64) {
    // dereference with "*"
    *x = 22.03
}

fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}