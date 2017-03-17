use std::fmt;
use std::env;

struct Swagger<T>(pub T);

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yolo {} swag", self.0)
    }
}

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self>;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
        Swagger(self)
    }
}


fn main() {
    for arg in env::args().skip(1) {
        let swag = Swagger(arg);
        println!("{}", swag);
    }

    println!("{}", 3.with_swag());    // prints: "yolo 3 swag"
}
