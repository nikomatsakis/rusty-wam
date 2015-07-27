struct Recorder {
    ops: Vec<String>,
    registers: usize,
}

impl Recorder {
    fn new() -> Recorder {
        Recorder { ops: vec![], registers: 0 }
    }
}

impl MachineOps for Recorder {
    fn next_register(&mut self) -> Register {
        self.registers += 1;
        Register(self.registers - 1)
    }

    fn put_structure(&mut self, f: Functor, r: Register) {
        ops.push_str(format!("put_structure {:?},{:?}", f, r))
    }

    fn set_variable(&mut self, r: Register) {
        ops.push_str(format!("set_variable {:?}", r))
    }

    fn set_value(&mut self, r: Register) {
        ops.push_str(format!("set_value {:?}", r))
    }
}

pub fn test(term: &ast::Term, expected_reg: Register, expected_ops: Vec<String>) {
    let r = Recorder::new();
    let reg = super::query(&mut r, term);
    println!("Query {:?} in {:?} after {:#?}", term, expected_reg, expected_ops);
    assert_eq!(expected_reg, reg);
    for (expected_op, actual_op) in expected_ops.iter().zip(&r.ops) {
        assert_eq!(expected_op, actual_op);
    }
    assert_eq!(expected_ops.len(), r.ops.len());
}

#[test]
fn test1() {
    test(
        &Term::Application
}
