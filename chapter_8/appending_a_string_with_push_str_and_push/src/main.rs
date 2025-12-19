fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {s}");

    let mut t1= String::from("foo");
    let t2 = "bar";
    t1.push_str(t2);
    println!("t1 is {t1}");

    // concatenation with +
    let x1 = String::from("Hello, ");
    let x2 = String::from("world!");
    let x3 = x1 + &x2;
    println!("x3 is {x3}");

    let d1 = String::from("tic");
    let d2 = String::from("tac");
    let d3 = String::from("toe");

    let d = d1 + "-" + &d2 + "-" + &d3;
    println!("d is: {d}");

    let e1 = String::from("tic");
    let e2 = String::from("tac");
    let e3 = String::from("toe");
    let ee = format!("{e1}-{e2}-{e3}");
    println!("ee is {ee}");
}
