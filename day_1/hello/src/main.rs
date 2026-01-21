mod sum;
mod util;

fn main() {
    // marco
    println!("Hello, world!");

    let n = 5;
    let mut m = n;
    m = m + 2;

    println!("Sum: {}", sum::sum(n, m));
    println!("Add: {}", util::add::add(n, m));
}
