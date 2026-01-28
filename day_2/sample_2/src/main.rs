fn main() {
    // primitive type - stack
    let a = 5;
    let b = a;

    println!("Value of a: {}", a);
    println!("Value of b: {}", b);

    // reference: phuc tap
    let mut s1 = "Hello".to_string();
    s1 = format!("{} World", s1); // -> Hello World

    let s2 = s1;

    // println!("Value of a: {}", s1);
    println!("Value of s2: {}", s2);
}
