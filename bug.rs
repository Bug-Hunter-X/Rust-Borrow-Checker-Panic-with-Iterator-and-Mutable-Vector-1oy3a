fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());

    //This will cause a panic
    vec.push(4); //Error
}