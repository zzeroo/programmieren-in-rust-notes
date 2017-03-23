//! https://www.reddit.com/r/rust/comments/34dy5z/reference_counted_cycles/
//! https://github.com/rust-lang/rust/issues/24456

use std::cell::RefCell;
use std::rc::Rc;

struct NastyEchoDrop {
    c: char,    // Leaked Data
    really_bad: RefCell<Option<Rc<NastyEchoDrop>>>,
}

impl Drop for NastyEchoDrop {
    fn drop(&mut self) {
        println!("dropped: {}", self.c);
    }
}

fn main() {
    let a = Rc::new(NastyEchoDrop {
        c: 'a',    // Leaked Data
        really_bad: RefCell::new(None),
    });

    let b = a.clone();

    *a.really_bad.borrow_mut() = Some(b);
}
