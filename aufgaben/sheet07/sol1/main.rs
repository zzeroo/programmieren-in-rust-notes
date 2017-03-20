fn main() {
    let res: Vec<_> = (0..).map(|x| fib(x)).take(20).collect();
    println!("{:?}", res);
}

fn fib(n: u32) -> u32 {
    match () {
        () if n == 0 => 0,
        () if n <= 1 => 1,
        () => fib(n - 1) + fib(n - 2),
    }
}
