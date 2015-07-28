use functor::Functor;
use std::fmt::{Debug, Error, Formatter};
use std::iter::repeat;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Address {
    Heap(usize),
    Register(usize),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Structure(Slot),
    Ref(Slot),
    Functor(Functor),
    Uninitialized,
}

impl Memory {
    pub fn new(num_regs: usize) -> Memory {
        let registers = repeat(Cell::Uninitialized).take(num_regs).collect();
        Memory { heap: vec![], registers: registers }
    }

    pub fn heap(&self) -> &[Cell] {
        &self.heap
    }

    pub fn registers(&self) -> &[Cell] {
        &self.registers
    }

    pub fn next_slot(&self) -> Slot {
        Slot(self.heap.len())
    }

    pub fn push(&mut self, cell: Cell) {
        self.heap.push(cell);
    }

    pub fn load<P:Pointer>(&self, p: P) -> Cell {
        p.load(self)
    }

    pub fn load_functor<P:Pointer>(&self, p: P) -> Functor {
        match self.load(p) {
            Cell::Functor(f) => f,
            cell => panic!("load_functor got {:?} for {:?}", cell, p)
        }
    }

    pub fn store<P:Pointer>(&mut self, p: P, cell: Cell) {
        p.store(self, cell)
    }

    pub fn bind(&mut self, addr1: Address, addr2: Address) {
        println!("bind({:?}={:?}, {:?}={:?})",
                 addr1, self.load(addr1),
                 addr2, self.load(addr2));
        match (self.load(addr1), self.load(addr2)) {
            (Cell::Ref(_), cell2) => {
                self.store(addr1, cell2);
            }
            (cell1, Cell::Ref(_)) => {
                self.store(addr2, cell1);
            }
            (cell1, cell2) => {
                panic!("bind invoked with two non-ref addresses: {:?}=>{:?}, {:?}=>{:?}",
                       addr1, cell1, addr2, cell2);
            }
        }
    }

    pub fn unify(&mut self, addr1: Address, addr2: Address) -> Fallible {
        let mut stack = vec![];
        stack.push((addr1, addr2));
        while let Some((d1, d2)) = stack.pop() {
            let d1 = self.deref(d1);
            let d2 = self.deref(d2);
            if d1 == d2.to_address() {
                continue;
            }

            match (self.load(d1), self.load(d2)) {
                (Cell::Ref(_), _) |
                (_, Cell::Ref(_)) => {
                    self.bind(d1, d2);
                }

                (Cell::Structure(v1), Cell::Structure(v2)) => {
                    let f1 = self.load_functor(v1);
                    let f2 = self.load_functor(v2);
                    if f1 == f2 {
                        for i in 1..(f1.arity()+1) {
                            stack.push(((v1 + i).to_address(),
                                        (v2 + i).to_address()));
                        }
                    } else {
                        return Err(());
                    }
                }

                (cell1, cell2) => {
                    panic!("Unexpected cell kind encountered in unify: {:?}=>{:?}, {:?}=>{:?}",
                           d1, cell1, d2, cell2)
                }
            }
        }
        Ok(())
    }

    pub fn deref<P:Pointer+FromSlot>(&mut self, ptr: P) -> P {
        match self.load(ptr) {
            Cell::Ref(referent) => {
                let referent = P::from_slot(referent);
                if ptr == referent {
                    ptr
                } else {
                    self.deref(referent)
                }
            }
            Cell::Structure(_) | Cell::Functor(_) => {
                ptr
            }
            Cell::Uninitialized => {
                panic!("Access to uninitialized cell at {:?}", ptr)
            }
        }
    }
}

impl Slot {
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

pub trait Pointer: Copy+Clone+Debug+PartialEq {
    fn to_address(self) -> Address;
    fn to_slot(self) -> Option<Slot>;
    fn load(self, mem: &Memory) -> Cell;
    fn store(self, mem: &mut Memory, cell: Cell);
}

pub trait FromSlot {
    fn from_slot(slot: Slot) -> Self;
}

impl Pointer for Address {
    fn to_address(self) -> Address {
        self
    }

    fn to_slot(self) -> Option<Slot> {
        match self {
            Address::Heap(i) => Some(Slot(i)),
            Address::Register(_) => None,
        }
    }

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

impl FromSlot for Address {
    fn from_slot(slot: Slot) -> Address {
        slot.to_address()
    }
}

impl Pointer for Slot {
    fn to_address(self) -> Address {
        Address::Heap(self.0)
    }

    fn to_slot(self) -> Option<Slot> {
        Some(self)
    }

    fn load(self, mem: &Memory) -> Cell {
        mem.heap[self.0]
    }

    fn store(self, mem: &mut Memory, cell: Cell) {
        mem.heap[self.0] = cell;
    }
}

impl FromSlot for Slot {
    fn from_slot(slot: Slot) -> Slot {
        slot
    }
}

impl Pointer for Register {
    fn to_address(self) -> Address {
        Address::Register(self.0)
    }

    fn to_slot(self) -> Option<Slot> {
        None
    }

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

///////////////////////////////////////////////////////////////////////////
// MGU -- prints out the current contents of a cell as a MGU

pub struct MGU<'mem> {
    mem: &'mem Memory,
    addr: Address
}

impl<'mem> MGU<'mem> {
    pub fn new(mem: &'mem Memory, addr: Address) -> MGU<'mem> {
        MGU { mem: mem, addr: addr }
    }

    fn write<P:Pointer>(&self, fmt: &mut Formatter, ptr: P) -> Result<(), Error> {
        match self.mem.load(ptr) {
            Cell::Structure(mut slot) => {
                let functor = self.mem.load_functor(slot);
                try!(write!(fmt, "{}", functor.text()));
                if functor.arity() > 0 {
                    try!(write!(fmt, "("));
                    slot.bump();
                    self.write(fmt, slot);
                    for i in 1 .. functor.arity() {
                        slot.bump();
                        try!(write!(fmt, ","));
                        self.write(fmt, slot);
                    }
                    try!(write!(fmt, ")"));
                }
                Ok(())
            }
            Cell::Ref(referent) => {
                if referent.to_address() == ptr.to_address() {
                    write!(fmt, "?")
                } else {
                    self.write(fmt, referent)
                }
            }
            cell @ Cell::Functor(_) |
            cell @ Cell::Uninitialized => {
                panic!("MGU found odd format for cell: {:?}", cell)
            }
        }
    }
}

impl<'mem> Debug for MGU<'mem> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.write(fmt, self.addr)
    }
}

impl Debug for Memory {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        try!(writeln!(fmt, "Memory {{"));
        for (i, cell) in self.heap.iter().enumerate() {
            try!(writeln!(fmt, "  H{:?}: {:?}", i, cell));
        }
        try!(writeln!(fmt, ""));
        for (i, cell) in self.registers.iter().enumerate() {
            try!(writeln!(fmt, "  R{:?}: {:?}", i, cell));
        }
        writeln!(fmt, "}}")
    }
}
