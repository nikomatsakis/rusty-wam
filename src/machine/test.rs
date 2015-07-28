use super::{Fallible, Machine, MachineOps};
use super::mem::Register;

use interpret;

fn heap(machine: &Machine) -> Vec<String> {
    machine.mem.heap().iter()
                      .enumerate()
                      .map(|(idx, cell)| format!("H{}: {:?}", idx, cell))
                      .collect()
}

macro_rules! test_heap {
    ($m:expr, $($e:expr),*) => {
        {
            let actual = heap(&$m);
            let expected = vec![$($e.to_string()),*];
            match actual.iter().zip(expected.iter()).find(|&(a, e)| a != e) {
                Some((a, e)) => { panic!( "actual {:?} expected {:?}", a, e); }
                None => { }
            }
            assert_eq!(actual.len(), expected.len());
        }
    }
}

fn figure2_3<M:MachineOps>(machine: &mut M) {
    machine.put_structure(functor!(h/2), Register(2));
    machine.set_variable(Register(1));
    machine.set_variable(Register(4));
    machine.put_structure(functor!(f/1), Register(3));
    machine.set_value(Register(4));
    machine.put_structure(functor!(p/3), Register(0));
    machine.set_value(Register(1));
    machine.set_value(Register(2));
    machine.set_value(Register(3));
}

fn figure2_4<M:MachineOps>(machine: &mut M) -> Fallible {
    try!(machine.get_structure(functor!(p/3), Register(0)));
    machine.unify_variable(Register(1));
    machine.unify_variable(Register(2));
    machine.unify_variable(Register(3));
    try!(machine.get_structure(functor!(f/1), Register(1)));
    machine.unify_variable(Register(4));
    try!(machine.get_structure(functor!(h/2), Register(2)));
    try!(machine.unify_value(Register(3)));
    machine.unify_variable(Register(5));
    try!(machine.get_structure(functor!(f/1), Register(5)));
    machine.unify_variable(Register(6));
    try!(machine.get_structure(functor!(a/0), Register(6)));
    Ok(())
}

#[test]
fn exercise2_1() {
    let mut machine = Machine::new(5);
    figure2_3(&mut machine);

    test_heap!(
        machine,
        "H0: Structure(H1)",
        "H1: Functor(h/2)",
        "H2: Ref(H2)",
        "H3: Ref(H3)",
        "H4: Structure(H5)",
        "H5: Functor(f/1)",
        "H6: Ref(H3)",
        "H7: Structure(H8)",
        "H8: Functor(p/3)",
        "H9: Ref(H2)",
        "H10: Structure(H1)",
        "H11: Structure(H5)");
}

#[test]
fn exercise2_3() {
    let mut machine = Machine::new(7);
    figure2_3(&mut machine);
    figure2_4(&mut machine).unwrap();
    assert_eq!(
        &format!("{:?}", machine.mgu(Register(0))),
        "p(f(f(a)),h(f(f(a)),f(a)),f(f(a)))");
}

#[test]
fn exercise2_3_1() {
    // same as above, but using `interpret` to drive the machine,
    // instead of a hardcoded sequence of instructions

    let mut machine = Machine::new(7);

    {
        let mut machine = machine.dump();
        interpret::query(&mut machine,   &structure!(p(?Z,    h(?Z, ?W),   f(?W))));
        interpret::program(&mut machine, &structure!(p(f(?X), h(?Y, f(a)), ?Y))).unwrap();
    }

    assert_eq!(
        &format!("{:?}", machine.mgu(Register(0))),
        "p(f(f(a)),h(f(f(a)),f(a)),f(f(a)))");
}

#[test]
fn fail_to_unify1() {
    // same as above, but using `interpret` to drive the machine,
    // instead of a hardcoded sequence of instructions

    let mut machine = Machine::new(7);
    interpret::query(&mut machine,   &structure!(p(?Z,    ?Z)));
    assert!(interpret::program(&mut machine, &structure!(p(f(?X), g(?X)))).is_err());
}

#[test]
fn exercise2_3_5() {
    // same as above, but using `interpret` to drive the machine,
    // instead of a hardcoded sequence of instructions

    let mut machine = Machine::new(7);
    interpret::query(&mut machine,   &structure!(p(f(?X), h(?Y, f(a)), ?Y)));
    interpret::program(&mut machine, &structure!(p(?Z,    h(?Z, ?W),   f(?W)))).unwrap();

    assert_eq!(
        &format!("{:?}", machine.mgu(Register(0))),
        "p(f(f(a)),h(f(f(a)),f(a)),f(f(a)))");
}
