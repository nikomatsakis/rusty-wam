//! Definition of the WAM.

//! Definition of the WAM.

use functor::Functor;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Error, Formatter};

pub struct Machine {
    heap: Vec<Cell>,
    registers: Vec<Cell>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Uninitialized,
    Structure(usize),
    Ref(usize),
    Functor(Functor),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Register(pub usize);

impl Debug for Register {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "R{}", self.0)
    }
}

trait MachineOps {
    fn next_register(&mut self) -> Register;
    fn put_structure(&mut self, f: Functor, r: Register);
    fn set_variable(&mut self, r: Register);
    fn set_value(&mut self, r: Register);
}

impl MachineOps for Machine {
    fn next_register(&mut self) -> Register {
        self.registers.push(Cell::Uninitialized);
        Register(self.registers.len() - 1)
    }

    /// from tutorial figure 2.2
    fn put_structure(&mut self, f: Functor, r: Register) {
        let addr = self.heap.len();
        let cell = Cell::Structure(addr + 1);
        self.heap.push(cell.clone());
        self.heap.push(Cell::Functor(f));
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    fn set_variable(&mut self, r: Register) {
        let addr = self.heap.len();
        let cell = Cell::Ref(addr);
        self.heap.push(cell.clone());
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    fn set_value(&mut self, r: Register) {
        let addr = self.heap.len();
        self.heap.push(self.registers[r.0].clone());
    }
}
