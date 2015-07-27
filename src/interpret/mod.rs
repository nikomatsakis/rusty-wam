//! Interpret AST terms into machine operations.

use ast::{Structure, Term};
use machine::{MachineOps, Register};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

#[cfg(test)]
mod test;

pub fn query<M:MachineOps>(machine: &mut M, structure: &Structure) {
    let mut interpreter = QueryInterpreter { machine: machine,
                                             registers: 1,
                                             map: HashMap::new(),
                                             generated: HashSet::new() };
    interpreter.structure(structure, Register(0));
}

pub struct QueryInterpreter<'query, M:MachineOps+'query> {
    registers: usize,
    machine: &'query mut M,
    map: HashMap<&'query Term, Register>,
    generated: HashSet<Register>,
}

impl<'query, M:MachineOps> QueryInterpreter<'query, M> {
    fn structure(&mut self, structure: &'query Structure, into: Register) {
        // The ordering here is "reverse engineered" from the
        // tutorial, which (somewhat surprisingly) doesn't specify it.

        // first, allocate registers for every term we see
        let term_registers: Vec<_> =
            structure.terms.iter()
                           .map(|term| self.register(term))
                           .collect();

        // next, recursively generate new structures (but not
        // variables); since queries are built bottom-up, this must be
        // done before generating the current term
        for (term, &reg) in structure.terms.iter().zip(&term_registers) {
            match *term {
                Term::Structure(ref substructure) => {
                    if self.generated.insert(reg) {
                        self.structure(substructure, reg);
                    }
                }

                Term::Variable(_) => { }
            }
        }

        // finally, build this term; structures will always have been
        // generated, but variables may or may not have been observed
        // yet
        self.machine.put_structure(structure.functor, into);
        for (term, &reg) in structure.terms.iter().zip(&term_registers) {
            match *term {
                Term::Structure(_) => {
                    debug_assert!(self.generated.contains(&reg));
                    self.machine.set_value(reg);
                }

                Term::Variable(_) => {
                    if self.generated.insert(reg) {
                        self.machine.set_variable(reg);
                    } else {
                        self.machine.set_value(reg);
                    }
                }
            }
        }
    }

    fn register(&mut self, term: &'query Term) -> Register {
        match self.map.entry(term) {
            // already have a register for this term; no work to do
            Entry::Occupied(slot) => {
                *slot.get()
            }

            // need a register
            Entry::Vacant(slot) => {
                let register = Register(self.registers);
                self.registers += 1;
                slot.insert(register);
                register
            }
        }
    }
}



