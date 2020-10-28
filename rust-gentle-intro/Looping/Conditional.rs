// looping with a conditional

fn main() {
    for i in 0..5 {
        if i % 2 == 0 {
            println!("{} is an even number", i);
        } else {
            println!("{} is an odd number", i);
        }
    }
}

// Below does the same thing.

/*
fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}
*/