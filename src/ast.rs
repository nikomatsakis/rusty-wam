use functor::Functor;
use intern::InternedString;
use std::fmt::{Debug, Display, Error, Formatter};

pub enum Term {
    Variable(InternedString),
    Application(Functor, Vec<Term>),
}

impl Debug for Term {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Term::Variable(t) => write!(fmt, "{}", t),
            Term::Application(f, ref terms) if terms.is_empty() => write!(fmt, "{}", f.text()),
            Term::Application(f, ref terms) => write!(fmt, "{}{:?}", f.text(), terms),
        }
    }
}

