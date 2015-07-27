use ast;
use machine::{MachineOps, Register};
use functor::Functor;

struct Recorder {
    ops: Vec<String>,
}

impl Recorder {
    fn new() -> Recorder {
        Recorder { ops: vec![] }
    }
}

impl MachineOps for Recorder {
    fn put_structure(&mut self, f: Functor, r: Register) {
        self.ops.push(format!("put_structure {:?},{:?}", f, r))
    }

    fn set_variable(&mut self, r: Register) {
        self.ops.push(format!("set_variable {:?}", r))
    }

    fn set_value(&mut self, r: Register) {
        self.ops.push(format!("set_value {:?}", r))
    }
}

pub fn test_query(structure: &ast::Structure, expected_ops: Vec<&str>) {
    let mut r = Recorder::new();
     super::query(&mut r, structure);
    println!("Query {:?} yields {:#?}", structure, r.ops);
    for (expected_op, actual_op) in expected_ops.iter().zip(&r.ops) {
        assert_eq!(expected_op, actual_op);
    }
    assert_eq!(expected_ops.len(), r.ops.len());
}

#[test]
fn test1() {
    test_query(
        &structure!(p(?Z,h(?Z,?W),f(?W))),
        vec![
    "put_structure h/2,R2",
    "set_variable R1",
    "set_variable R4",
    "put_structure f/1,R3",
    "set_value R4",
    "put_structure p/3,R0",
    "set_value R1",
    "set_value R2",
    "set_value R3"
            ]);
}
