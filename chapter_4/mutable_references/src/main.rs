fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");

    let r3 = &mut s; // no problem
    println!("{r3}")
}
