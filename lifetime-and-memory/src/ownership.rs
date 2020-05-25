pub fn init() {
    println!("Ownership:");
    let _u1 = Box::new(1); // Assigned on heap
    let _u2 = 1;

    let wew = |x: Vec<i32>| -> Vec<i32> { x.iter().map(|v| *v + 1).collect() };

    let x = vec![3, 6, 5, 2];
    println!("{:?}", wew(x))
}
