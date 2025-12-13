fn main() {
    // Compound types can group multiple values into one type
    // Rust has 2 compound types: tuples and arrays

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of y is {y}");

    let x: (i32,f64,u8) = (500,6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    let array = [1,2,3,4,5]; // arrays are useful when you want to allocate on stack vs heap
    // unlike a tuple every element of an array must be the same type
    // In Rust, arrays have a fixed length
    // Vector are similar to arrays, but are allowed to grow or shrink in length

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,2,3,4,5];

    // you can also intialize an to contain the same value for each element
    let aa = [3; 5];

    // accessing array elements
    let aaa = [1,2,3,4,5];
    let first = aaa[0];
    let second = aaa[1];

}
