use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        // SAFETY: By implementing UnsafeCell we've made cell !sync there for it can only be usd by a single thread
        // SAFETY: Get method returns a copy so no danger in mutating this value as no one is borrowing it 
       unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T 
    where 
        T: Copy, 
    {
         // SAFETY: By implementing UnsafeCell we've made cell !sync there for it can only be usd by a single thread
         // SAFETY: Rewuiring value to have Copy triat so we never explose the value directly
        unsafe { *self.value.get() }
    }
}