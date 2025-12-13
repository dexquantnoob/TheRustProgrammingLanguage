fn main() {
    let x = five();
    println!("The value of x is {x}");
    let x = plus_one(x);
    println!("The value of x is {x}");
}

fn five() -> i32 {
    5  // return values dont have a ; 
}

fn plus_one(x:i32) -> i32 {
    x + 1 // return values dont have a ; 
}