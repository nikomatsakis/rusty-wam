//! Interpret AST terms into machine operations.

use ast::Term;
use std::collections::HashMap;
use std::collections::hash::Entry;

#[cfg(test)]
mod test;

pub fn query<M:MachineOps>(machine: &mut M, term: &Term) -> Register {
    let interpreter = QueryInterpreter { machine: machine, map: HashMap::new() };
    interpreter.query(term)
}

pub struct QueryInterpreter<M:MachineOps> {
    machine: M,
    map: HashMap<InternedString, Register>,
}

impl<M:MachineOps> QueryInterpreter<M> {
    fn query(&mut self, term: &Term) -> Register {
        match *self {
            Term::Variable(v) =>
                match map.entry(v) {
                    Entry::Vacant(slot) => {
                        let register = self.machine.next_register();
                        slot.insert(r);
                        self.machine.set_variable(r);
                        register
                    }
                    Entry::Occupied(slot) => {
                        let register = *slot.get();
                        self.machine.set_value(r);
                        register
                    }
                },

            Term::Application(functor, ref terms) => {
                // Here I deviate slightly from the text as written in
                // the tutorial; I think that this text would maintain
                // the invariant that the argument to query-map is a
                // Structure, and recurse only on structures. This
                // seems to be strictly more annoying and I think has
                // the same effect, so I'm not quite sure why they do
                // it this way.

                let registers: Vec<_> =
                    terms.iter()
                         .map(|t| self.query(t))
                         .collect();

                let register = self.machine.next_register();
                self.machine.put_structure(functor, register);
                for register in registers {
                    self.machine.set_value(register);
                }
            }
        }
    }
}



