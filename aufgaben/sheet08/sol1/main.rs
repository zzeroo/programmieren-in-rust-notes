use std::str::FromStr;

trait RubbyExt<T> {
    fn times<F: FnMut(T)>(&self, f: F);
}

impl RubbyExt<i32> for i32 {
    fn times<F: FnMut(i32)>(&self, mut f: F) {
        for i in 0..*self {
            f(i);
        }
    }
}


fn main() {
    3.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
}
