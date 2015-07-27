//! Functor interning table.

use intern::InternedString;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Error, Formatter};

pub struct Functors {
    data: Vec<FunctorData>,
    map: HashMap<FunctorData, Functor>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Functor(usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorData {
    pub text: InternedString,
    pub arity: usize,
}

///////////////////////////////////////////////////////////////////////////
// TLS

thread_local! {
    static FUNCTOR_TLS: RefCell<Functors> =
        RefCell::new(Functors::new())
}

pub fn read<F,R>(f: F) -> R
    where F: FnOnce(&Functors) -> R
{
    FUNCTOR_TLS.with(|functors| f(&*functors.borrow()))
}

fn write<F,R>(f: F) -> R
    where F: FnOnce(&mut Functors) -> R
{
    FUNCTOR_TLS.with(|functors| f(&mut *functors.borrow_mut()))
}

pub fn functor(text: InternedString, arity: usize) -> Functor {
    write(|f| f.functor(text, arity))
}

///////////////////////////////////////////////////////////////////////////
// Functors table

impl Functors {
    fn new() -> Functors {
        Functors { map: HashMap::new(), data: vec![] }
    }

    pub fn functor(&mut self, text: InternedString, arity: usize) -> Functor {
        let data = FunctorData { text: text, arity: arity };
        match self.map.get(&data) {
            Some(&functor) => { return functor; }
            None => { }
        }

        let functor = Functor(self.data.len());
        self.map.insert(data, functor);
        functor
    }

    pub fn data(&self, f: Functor) -> &FunctorData {
        &self.data[f.0]
    }
}

///////////////////////////////////////////////////////////////////////////
// Methods on Functor

impl Functor {
    pub fn text(self) -> InternedString {
        read(|f| f.data(self).text)
    }

    pub fn arity(self) -> usize {
        read(|f| f.data(self).arity)
    }
}

impl Debug for Functor {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}/{}", self.text(), self.arity())
    }
}
