fn main() {
    let s1 = "meow";
    println!("Immutable {s1}");

    let mut s2 = String::from("meow");
    s2.push_str(" meow");

    println!("Mutable {s2}");

    let mut x = 5;
    let mut y = x;
    x += 1;
    y += 2;

    println!("updates {x} {y}");

    let s1 = String::from("hello s1");
    let mut s2 = s1;

    // s1.push_str("hello s1"); // can no longer access s1
    s2.push_str("hello s2");

    let mut s3 = s2.clone();
    s3.push_str("hello s3");

    println!("strings s2: {s2} | s3: {s3}"); // can still access s2 since we've cloned it
}
