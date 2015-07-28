//! Definition of the WAM.

use functor::Functor;
use std::fmt::{Debug, Error, Formatter};

use self::mem::{Cell, Memory, Pointer, Slot, Register};

pub mod mem;

#[cfg(test)]
mod test;

pub struct Machine {
    mem: Memory,
    mode: Mode,
}

enum Mode {
    Read(Slot),
    Write,
}

pub type Fallible = Result<(),()>;

pub trait MachineOps {
    fn put_structure(&mut self, f: Functor, r: Register);
    fn set_variable(&mut self, r: Register);
    fn set_value(&mut self, r: Register);

    fn get_structure(&mut self, f: Functor, r: Register) -> Fallible;
    fn unify_variable(&mut self, r: Register);
    fn unify_value(&mut self, r: Register) -> Fallible;
}

impl Machine {
    pub fn new(num_registers: usize) -> Machine {
        Machine { mem: Memory::new(num_registers), mode: Mode::Write }
    }

    pub fn mgu<'a,P:mem::Pointer>(&'a self, addr: P) -> mem::MGU<'a> {
        mem::MGU::new(&self.mem, addr.to_address())
    }
}

impl MachineOps for Machine {
    /// from tutorial figure 2.2
    fn put_structure(&mut self, f: Functor, r: Register) {
        let ptr = self.mem.next_slot();
        let cell = Cell::Structure(ptr + 1);
        self.mem.push(cell);
        self.mem.push(Cell::Functor(f));
        self.mem.store(r, cell);
    }

    /// from tutorial figure 2.2
    fn set_variable(&mut self, r: Register) {
        let ptr = self.mem.next_slot();
        let cell = Cell::Ref(ptr);
        self.mem.push(cell);
        self.mem.store(r, cell);
    }

    /// from tutorial figure 2.2
    fn set_value(&mut self, r: Register) {
        let cell = self.mem.load(r);
        self.mem.push(cell);
    }

    fn get_structure(&mut self, f: Functor, r: Register) -> Fallible {
        let addr = self.mem.deref(r.to_address());
        match self.mem.load(addr) {
            Cell::Ref(_) => {
                let slot = self.mem.next_slot();
                self.mem.push(Cell::Structure(slot + 1));
                self.mem.push(Cell::Functor(f));
                self.mem.bind(addr, slot);
                self.mode = Mode::Write;
                Ok(())
            }
            Cell::Structure(slot) => {
                if self.mem.load(slot) == Cell::Functor(f) {
                    let next = slot + 1;
                    self.mode = Mode::Read(next);
                    Ok(())
                } else {
                    // if the pointer doesn't reference a functor, heap is inconsistent
                    debug_assert!(match self.mem.load(slot) {
                        Cell::Functor(_) => true,
                        _ => false,
                    });
                    Err(())
                }
            }
            Cell::Functor(_) => {
                Err(())
            }
            Cell::Uninitialized => {
                panic!("Load from uninitialized cell at {:?}", addr)
            }
        }
    }

    fn unify_variable(&mut self, reg: Register) {
        match self.mode {
            Mode::Read(ref mut next) => {
                let cell = self.mem.load(*next);
                self.mem.store(reg, cell);
                next.bump();
            }

            Mode::Write => {
                let ptr = self.mem.next_slot();
                let cell = Cell::Ref(ptr);
                self.mem.push(cell);
                self.mem.store(reg, cell);
            }
        }
    }

    fn unify_value(&mut self, reg: Register) -> Fallible {
        match self.mode {
            Mode::Read(ref mut next) => {
                try!(self.mem.unify(reg.to_address(), *next));
                next.bump();
                Ok(())
            }
            Mode::Write => {
                let cell = self.mem.load(reg);
                self.mem.push(cell);
                Ok(())
            }
        }
    }
}

