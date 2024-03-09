use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

fn main() {
    let mut arr = Rc::new(RefCell::new(34));
    let mut refs = arr.borrow_mut();
    println!("{:?}", refs);
}
