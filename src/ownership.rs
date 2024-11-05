pub fn run() {
    let mut x: Vec<i32> = vec![1, 2, 3, 4];
    let foo = |x: &mut Vec<i32>| x.push(22);
    foo(&mut x);
    println!("x still maintains ownership of the data it points to because we passed stuff as reference: {:?}", x);
}
