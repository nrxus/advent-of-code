use intcode::*;

#[test]
fn print_program() {
    let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();

    let mut machine = Machine::new(codes.clone());

    let mut outputs = vec![];
    while let MachineResult::HasOutput(output) = machine.execute() {
        let (out, m) = output.read();
        machine = m;
        outputs.push(out);
    }

    assert_eq!(codes.into_iter().map(|i| i.0).collect::<Vec<_>>(), outputs);
}

#[test]
fn large_number() {
    let machine = Machine::from_str("1102,34915192,34915192,7,4,7,99,0");

    match machine.execute() {
        MachineResult::HasOutput(output) => {
            let (out, m) = output.read();
            assert_eq!(out, 1219070632396864);
            m.run_to_halt().expect("did not halt");
        }
        _ => panic!("expected output"),
    }
}

#[test]
fn prints_middle_number() {
    let machine = Machine::from_str("104,1125899906842624,99");

    match machine.execute() {
        MachineResult::HasOutput(output) => {
            let (out, m) = output.read();
            assert_eq!(out, 1125899906842624);
            m.run_to_halt().expect("did not halt");
        }
        _ => panic!("expected output"),
    }
}
