fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Solution 1: Clone the vector
    let cloned_vec = vec.clone();
    let mut iter = cloned_vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());
    vec.push(4);

    //Solution 2: Consume the iterator
    let mut iter = vec.into_iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());
    //After consuming the iterator, we can freely modify vec
    vec.push(4);

    //Solution 3: Use a for loop to iterate and modify after
    for i in &vec {
        println!("Element: {}", i);
    }
    vec.push(4);
}