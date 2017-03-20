#![feature(step_by)]
fn main() {
    // product
    let fac_5 = (1..5).fold(1, |acc, x| acc * x);
    assert_eq!(fac_5, (1..5).product());
    assert_eq!(fac_5, 24);

    // max
    let max = vec![3, 1, 4, 1, 5, 9, 2, 6].into_iter().fold(0, |acc, x| {
        std::cmp::max(acc, x)
    });
    assert_eq!(max, 9);
    println!("{}", max);

    // all
    let all_even = (2..12).step_by(2).fold(true, |acc, x| {
        acc && ( x % 2 == 0)
    });
    assert!(all_even);
    println!("{}", all_even);
}
