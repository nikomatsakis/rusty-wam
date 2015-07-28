use functor::Functor;
use intern::InternedString;
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Variable(InternedString),
    Structure(Structure),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Structure {
    pub functor: Functor,
    pub terms: Vec<Term>
}

impl Debug for Term {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Term::Variable(t) => write!(fmt, "?{}", t),
            Term::Structure(ref s) => write!(fmt, "{:?}", s),
        }
    }
}

impl Debug for Structure {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.terms.is_empty() {
            write!(fmt, "{}", self.functor.text())
        } else {
            try!(write!(fmt, "{}", self.functor.text()));
            let mut sep = '(';
            for term in &self.terms {
                try!(write!(fmt, "{}{:?}", sep, term));
                sep = ',';
            }
            write!(fmt, ")")
        }
    }
}

macro_rules! term {
    ($($args:tt)*) => {
        {
            let (term, ()) = terms_tt!($($args,)*,,);
            term
        }
    }
}

macro_rules! structure {
    ($($args:tt)*) => {
        match terms_tt!($($args,)*,,) {
            ($crate::ast::Term::Structure(s), ()) => s,
            (r @ $crate::ast::Term::Variable(_), ()) => panic!("{:?} is not a structure", r),
        }
    }
}

macro_rules! terms_tt {
    () => {
        ()
    };

    (?, $x:ident, ,, $($remainder:tt,)*) => {
        ($crate::ast::Term::Variable($crate::intern::intern(stringify!($x))),
         terms_tt!($($remainder,)*))
    };

    ($x:ident, ($($args:tt)*), ,, $($remainder:tt,)*) => {
        (
            {
                let mut vec = vec![];
                let args = terms_tt!($($args,)*,,);
                $crate::ast::ToTermVec::push_to_term_vec(args, &mut vec);
                let name = $crate::intern::intern(stringify!($x));
                let functor = $crate::functor::Functor::new(name, vec.len());
                $crate::ast::Term::Structure($crate::ast::Structure {
                    functor: functor,
                    terms: vec
                })
            },
            terms_tt!($($remainder,)*)
        )
    };

    ($x:ident, ,, $($remainder:tt,)*) => {
        (
            {
                let name = $crate::intern::intern(stringify!($x));
                let functor = $crate::functor::Functor::new(name, 0);
                $crate::ast::Term::Structure($crate::ast::Structure {
                    functor: functor,
                    terms: vec![]
                })
            },
            terms_tt!($($remainder,)*)
        )
    };
}

pub trait ToTermVec {
    fn push_to_term_vec(self, v: &mut Vec<Term>);
}

impl ToTermVec for () {
    fn push_to_term_vec(self, _: &mut Vec<Term>) {
    }
}

impl<T:ToTermVec> ToTermVec for (Term, T) {
    fn push_to_term_vec(self, v: &mut Vec<Term>) {
        v.push(self.0);
        self.1.push_to_term_vec(v);
    }
}

#[cfg(test)]
mod test;

