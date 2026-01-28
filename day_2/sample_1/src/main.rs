fn add_number(mut n: i32) -> i32 {
    // mut: mutuable
    n = n + 10;
    n
}

fn main() {
    let a = 5;
    let b = add_number(a);

    println!("Gia tri cua a la {}", a);
    println!("Gia tri cuar b la {}", b);

    // let a = 5;
    // let b = a; // 15

    // println!("Value of a: {}", a);
    // println!("Value of b: {}", b);

    // let s1 = "Hello".to_string();
    // let s2 = "World";
    // let s2 = &s1;

    // println!("Value of s1: {}", s1); // Hello
    // println!("Value of s2: {}", s2); // Hello

    // let r = 5;
    // {
    //     let r2: i32 = 1;
    //     let r = &r2;
    //     println!("Value of r: {}", r);
    // }
    // // -- r2: value
    // println!("Value of r: {}", r);

    // reference: Heap ---
    // use &mut
    let mut s1 = "Hi".to_string();
    s1 = format!("{} One", s1); //->  Hi One
    println!("s1: {}", s1);

    let s3 = &s1;
    println!("s3: {}", s3);

    // su dung &mut
    let s2 = &mut s1;
    *s2 = format!("{} and Two", s2); // deref
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // Rule 1: cho phép nhiều readers
    // Rule 3: Không thể có `&mut T` và `&T` cùng lúc
    {
        let s11 = &s1;
        let s12 = &s1;
        println!("s11: {}, s12: {}", s11, s12);
    }

    // Rule 2: một writer duy nhất
    {
        let s3 = &mut s1;
        *s3 = format!("{} and Three", s3);
        println!("s3 {}", s3);
    }

    // uncomment out to see errors
    // println!("s2 {}", s2);
}
