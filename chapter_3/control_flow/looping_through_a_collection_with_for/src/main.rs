fn main() {
    some_while();
    some_for();
    liftoff();

}
fn some_while() {
    {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5{
        println!("the value is: {}", a[index]);
        index += 1;
    }
}
}

fn some_for() {
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn liftoff() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!")
}