fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third of Vector 'v' element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element of Vector 'v' is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let mut x = vec![1,2,3,4,5];
    let first = &v[0];
    x.push(6);
    println!("The first element of Vector 'x' is: {first}");


}
