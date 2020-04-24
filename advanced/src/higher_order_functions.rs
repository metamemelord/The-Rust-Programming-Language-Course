fn upper_bound_checker(ub: i32) -> impl Fn(i32) -> bool {
    move |x| x < ub
}

fn even(x: &i32) -> bool {
    x % 2 == 0
}

pub fn init() {
    println!("\nHigher order functions:");
    let ub = 100;

    let mut iter = 0;
    let mut sum: i32 = 0;

    let hundred_upper_bound_checker = upper_bound_checker(ub);

    while hundred_upper_bound_checker(iter) {
        if even(&iter) {
            sum += iter * iter;
        }
        iter += 1;
    }
    println!("{}", sum);

    let s2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < ub)
        .filter(even)
        .fold(0, |s, x| s + x);
    println!("From higher order function {}", s2);
}
