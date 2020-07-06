use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
pub enum RefState {
    Unshared,
    Shared(usize),
    Exclusive
}

pub struct RefCell<T> {
    pub value: UnsafeCell<T>,
    pub state: Cell<RefState>
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell { 
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared)
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                // SAFETY: No exclusive references have been given since state is not exclusive
                self.state.set(RefState::Shared(1));
                Some( Ref{ refcell: self } )
            },
            RefState::Shared(n) => {
                // SAFETY: No exclusive references have been given since state is not exclusive
                self.state.set(RefState::Shared(n+1));
                Some( Ref{ refcell: self } )
            },
            RefState::Exclusive => None
        }
    }

    pub fn borrow_mut(&mut self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            // SAFETY: No references have been given since state unshared
            self.state.set(RefState::Exclusive);
            return Some( RefMut{ refcell: self } )
        }
        None
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Exclusive => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            },
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared)
            }
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}