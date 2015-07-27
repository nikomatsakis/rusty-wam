use super::{Machine, MachineOps};
use super::mem::Register;

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

#[test]
fn figure2_3() {
    let mut machine = Machine::new(5);
    machine.put_structure(functor!(h/2), Register(2));
    machine.set_variable(Register(1));
    machine.set_variable(Register(4));
    machine.put_structure(functor!(f/1), Register(3));
    machine.set_value(Register(4));
    machine.put_structure(functor!(p/3), Register(0));
    machine.set_value(Register(1));
    machine.set_value(Register(2));
    machine.set_value(Register(3));

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
