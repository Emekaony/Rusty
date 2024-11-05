pub fn run() {
    let mut x: Vec<i32> = vec![1, 2, 3, 4];
    let foo = |x: &mut Vec<i32>| x.push(22);
    foo(&mut x);
    println!("x still maintains ownership of the data it points to because we passed stuff as reference: {:?}", x);

    // borrowing
    let mut x = vec![1, 2, 3];
    let borrowed_x = &x; // so I have borrowed x now, but still not mutable
    println!("borrowed x: {:?}", borrowed_x);
    let mutably_borrowed_x = &mut x; // this will fail because we cannot have mutable and immutable borrowing of one variable in the same scope!!!!
    println!("this will fail when we try to access x againa: {:?}", x);
}
