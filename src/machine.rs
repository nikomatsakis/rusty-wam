//! Definition of the WAM.

use functor::Functor;
use std::collections::HashMap;

pub struct Machine {
    pub heap: Vec<Cell>,
    pub registers: Vec<Cell>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Structure(usize),
    Ref(usize),
    Functor(Functor),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Register(pub usize);
