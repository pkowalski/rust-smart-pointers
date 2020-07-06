mod cell;
mod refcell;

use cell::Cell;

use refcell::{RefCell};

fn main() {
    let some_cell = Cell::new(42);
    println!("{}", some_cell.get());
    some_cell.set(43);
    println!("{}", some_cell.get());

    let mut ref_cell = RefCell::new(30);
    {
        let some_ref = ref_cell.borrow().unwrap();
        println!("{}", *some_ref);
    }
    {
        let mut mut_ref = ref_cell.borrow_mut().unwrap();
        *mut_ref = 66;
    }
    let some_ref = ref_cell.borrow().unwrap();
    println!("{}", *some_ref);
}
