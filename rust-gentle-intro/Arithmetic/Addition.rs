// Addition and mutation

fn main() {
    // sum needs to be set as mut or the variable is unmutable
    // let sum = 0;
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("The number is {}", sum);
}