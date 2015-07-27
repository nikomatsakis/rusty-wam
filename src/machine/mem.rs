use functor::Functor;
use std::fmt::{Debug, Error, Formatter};
use std::ops;

use super::Fallible;

pub struct Memory {
    heap: Vec<Cell>,
    registers: Vec<Cell>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Slot(usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Register(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Address {
    Heap(usize),
    Register(usize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Structure(Slot),
    Ref(Slot),
    Functor(Functor),
}

impl Memory {
    pub fn next_slot(&self) -> Slot {
        Slot(self.heap.len())
    }

    pub fn push(&mut self, cell: Cell) {
        self.heap.push(cell);
    }

    pub fn load<P:Pointer>(&self, p: P) -> Cell {
        p.load(self)
    }

    pub fn store<P:Pointer>(&mut self, p: P, cell: Cell) {
        p.store(self, cell)
    }

    pub fn bind(&mut self, _addr1: Address, _addr2: Address) -> Fallible {
        panic!("NYI")
    }

    pub fn unify(&mut self, _addr1: Address, _addr2: Address) -> Fallible {
        panic!("NYI")
    }

    pub fn deref(&mut self, addr: Address) -> Address {
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
}

impl Register {
    pub fn to_address(self) -> Address {
        Address::Register(self.0)
    }
}

impl Slot {
    pub fn to_address(self) -> Address {
        Address::Heap(self.0)
    }

    pub fn bump(&mut self) {
        self.0 += 1;
    }
}

impl ops::Add<usize> for Slot {
    type Output = Slot;

    fn add(self, other: usize) -> Slot {
        Slot(self.0 + other)
    }
}

///////////////////////////////////////////////////////////////////////////
// Load and store

trait Pointer {
    fn load(self, mem: &Memory) -> Cell;
    fn store(self, mem: &mut Memory, cell: Cell);
}

impl Pointer for Address {
    fn load(self, mem: &Memory) -> Cell {
        match self {
            Address::Heap(i) => mem.heap[i],
            Address::Register(i) => mem.registers[i],
        }
    }

    fn store(self, mem: &mut Memory, cell: Cell) {
        match self {
            Address::Heap(i) => mem.heap[i] = cell,
            Address::Register(i) => mem.registers[i] = cell,
        }
    }
}

impl Pointer for Slot {
    fn load(self, mem: &Memory) -> Cell {
        mem.heap[self.0]
    }

    fn store(self, mem: &mut Memory, cell: Cell) {
        mem.heap[self.0] = cell;
    }
}

impl Pointer for Register {
    fn load(self, mem: &Memory) -> Cell {
        mem.registers[self.0]
    }

    fn store(self, mem: &mut Memory, cell: Cell) {
        mem.registers[self.0] = cell;
    }
}

impl Debug for Register {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "R{}", self.0)
    }
}

impl Debug for Slot {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "H{}", self.0)
    }
}

