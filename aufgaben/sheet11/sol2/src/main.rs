extern crate ttt;

use ttt::*;


fn main() {
    let ki = Ki::new(KiType::thumb);

    let i3 = Matrix::<N3, N3>::new_map(|i, j| if i == j { 1.0 } else { 0.0 });
    println!("{:?}", i3);
}
