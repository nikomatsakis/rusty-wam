//! Definition of the WAM.

use functor::Functor;
use std::fmt::{Debug, Error, Formatter};
use std::ops;

pub struct Machine {
    heap: Vec<Cell>,
    registers: Vec<Cell>,
    s: Pointer,
    mode: Mode
}

enum Mode {
    Read,
    Write,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Structure(Pointer),
    Ref(Pointer),
    Functor(Functor),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pointer(usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Register(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Address {
    Heap(usize),
    Register(usize),
}

impl Debug for Register {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "R{}", self.0)
    }
}

impl Debug for Pointer {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "H{}", self.0)
    }
}

pub type Fallible = Result<(),()>;

pub trait MachineOps {
    fn put_structure(&mut self, f: Functor, r: Register);
    fn set_variable(&mut self, r: Register);
    fn set_value(&mut self, r: Register);

    fn get_structure(&mut self, f: Functor, r: Register) -> Fallible;
    fn unify_variable(&mut self, r: Register) -> Fallible;
    fn unify_value(&mut self, r: Register) -> Fallible;
}

impl MachineOps for Machine {
    /// from tutorial figure 2.2
    fn put_structure(&mut self, f: Functor, r: Register) {
        let ptr = Pointer(self.heap.len());
        let cell = Cell::Structure(ptr + 1);
        self.heap.push(cell);
        self.heap.push(Cell::Functor(f));
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    fn set_variable(&mut self, r: Register) {
        let ptr = Pointer(self.heap.len());
        let cell = Cell::Ref(ptr);
        self.heap.push(cell);
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    fn set_value(&mut self, r: Register) {
        self.heap.push(self.registers[r.0]);
    }

    fn get_structure(&mut self, f: Functor, r: Register) -> Fallible {
        let addr = self.deref(r.to_address());
        match self.load(addr) {
            Cell::Ref(_) => {
                let ptr = Pointer(self.heap.len());
                self.heap.push(Cell::Structure(ptr + 1));
                self.heap.push(Cell::Functor(f));
                try!(self.bind(addr, ptr.to_address()));
                self.mode = Mode::Write;
                Ok(())
            }
            Cell::Structure(pointer) => {
                if self.heap[pointer.0] == Cell::Functor(f) {
                    self.s = Pointer(pointer.0 + 1);
                    self.mode = Mode::Read;
                    Ok(())
                } else {
                    // if the pointer doesn't reference a functor, heap is inconsistent
                    debug_assert!(match self.heap[pointer.0] {
                        Cell::Functor(_) => true,
                        _ => false,
                    });
                    Err(())
                }
            }
            Cell::Functor(_) => {
                Err(())
            }
        }
    }

    fn unify_variable(&mut self, reg: Register) -> Fallible {
        panic!("NYI")
    }

    fn unify_value(&mut self, reg: Register) -> Fallible {
        panic!("NYI")
    }
}

impl Register {
    fn to_address(self) -> Address {
        Address::Register(self.0)
    }
}

impl Pointer {
    fn to_address(self) -> Address {
        Address::Heap(self.0)
    }
}

impl ops::Add<usize> for Pointer {
    type Output = Pointer;

    fn add(self, other: usize) -> Pointer {
        Pointer(self.0 + other)
    }
}

impl Machine {
    fn load(&self, addr: Address) -> Cell {
        match addr {
            Address::Heap(i) => self.heap[i],
            Address::Register(i) => self.registers[i],
        }
    }

    fn store(&mut self, addr: Address, cell: Cell) {
        match addr {
            Address::Heap(i) => self.heap[i] = cell,
            Address::Register(i) => self.registers[i] = cell,
        }
    }

    fn deref(&mut self, addr: Address) -> Address {
        match self.load(addr) {
            Cell::Ref(referent) => {
                let referent = referent.to_address();
                if addr == referent {
                    addr
                } else {
                    let result = self.deref(referent);
                    result
                }
            }
            Cell::Structure(_) | Cell::Functor(_) => {
                return addr;
            }
        }
    }

    fn bind(&mut self, addr1: Address, addr2: Address) -> Fallible {
        panic!("NYI")
    }
}
