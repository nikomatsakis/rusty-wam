//! Primitive machine operations.

use functor::Functor;
use machine::*;

impl Machine {
    /// from tutorial figure 2.2
    pub fn put_structure(&mut self, f: Functor, r: Register) {
        let addr = self.heap.len();
        let cell = Cell::Structure(addr + 1);
        self.heap.push(cell.clone());
        self.heap.push(Cell::Functor(f));
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    pub fn set_variable(&mut self, r: Register) {
        let addr = self.heap.len();
        let cell = Cell::Ref(addr);
        self.heap.push(cell.clone());
        self.registers[r.0] = cell;
    }

    /// from tutorial figure 2.2
    pub fn set_value(&mut self, r: Register) {
        let addr = self.heap.len();
        self.heap.push(self.registers[r.0].clone());
    }
}
