//! Interpret AST terms into machine operations.

use ast::{Structure, Term};
use intern::InternedString;
use machine::{Fallible, MachineOps};
use machine::mem::Register;
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

pub struct QueryInterpreter<'term, M:MachineOps+'term> {
    registers: usize,
    machine: &'term mut M,
    map: HashMap<InternedString, Register>,
    generated: HashSet<InternedString>,
}

impl<'term, M:MachineOps> QueryInterpreter<'term, M> {
    fn structure(&mut self, structure: &'term Structure, into: Register) {
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
                    self.structure(substructure, reg);
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
                    self.machine.set_value(reg);
                }

                Term::Variable(v) => {
                    if self.generated.insert(v) {
                        self.machine.set_variable(reg);
                    } else {
                        self.machine.set_value(reg);
                    }
                }
            }
        }
    }

    fn register(&mut self, term: &'term Term) -> Register {
        match *term {
            Term::Structure(_) => {
                let register = bump_register(&mut self.registers);
                register
            }
            Term::Variable(v) => {
                match self.map.entry(v) {
                    // already have a register for this term; no work to do
                    Entry::Occupied(slot) => {
                        *slot.get()
                    }

                    // need a register
                    Entry::Vacant(slot) => {
                        let register = bump_register(&mut self.registers);
                        slot.insert(register);
                        register
                    }
                }
            }
        }
    }
}

pub fn program<M:MachineOps>(machine: &mut M, structure: &Structure) -> Fallible {
    let mut interpreter = ProgramInterpreter { machine: machine,
                                               registers: 1,
                                               map: HashMap::new(),
                                               generated: HashSet::new() };
    interpreter.structure(structure, Register(0))
}

pub struct ProgramInterpreter<'term, M:MachineOps+'term> {
    registers: usize,
    machine: &'term mut M,
    map: HashMap<InternedString, Register>,
    generated: HashSet<InternedString>,
}

impl<'term, M:MachineOps> ProgramInterpreter<'term, M> {
    fn structure(&mut self, structure: &'term Structure, into: Register) -> Fallible {
        // The ordering here is "reverse engineered" from the
        // tutorial, which (somewhat surprisingly) doesn't specify it.

        // first, allocate registers for every term we see
        let term_registers: Vec<_> =
            structure.terms.iter()
                           .map(|term| self.register(term))
                           .collect();

        // finally, build this term; structures will never have been
        // generated, but variables may or may not have been observed
        // yet
        try!(self.machine.get_structure(structure.functor, into));
        for (term, &reg) in structure.terms.iter().zip(&term_registers) {
            match *term {
                Term::Structure(_) => {
                    self.machine.unify_variable(reg);
                }

                Term::Variable(v) => {
                    if self.generated.insert(v) {
                        self.machine.unify_variable(reg);
                    } else {
                        try!(self.machine.unify_value(reg));
                    }
                }
            }
        }

        // next, recursively generate new structures (but not
        // variables); since programs are built top-down, this must be
        // done after generating the current term
        for (term, &reg) in structure.terms.iter().zip(&term_registers) {
            match *term {
                Term::Structure(ref substructure) => {
                    try!(self.structure(substructure, reg));
                }

                Term::Variable(_) => { }
            }
        }

        Ok(())
    }

    fn register(&mut self, term: &'term Term) -> Register {
        match *term {
            Term::Structure(_) => {
                let register = bump_register(&mut self.registers);
                register
            }
            Term::Variable(v) => {
                match self.map.entry(v) {
                    // already have a register for this term; no work to do
                    Entry::Occupied(slot) => {
                        *slot.get()
                    }

                    // need a register
                    Entry::Vacant(slot) => {
                        let register = bump_register(&mut self.registers);
                        slot.insert(register);
                        register
                    }
                }
            }
        }
    }
}

fn bump_register(registers: &mut usize) -> Register {
    let r = Register(*registers);
    *registers += 1;
    r
}

