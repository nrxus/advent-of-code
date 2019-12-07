use std::convert::TryInto;

fn solve(input: &str) -> i32 {
    let mut codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
    let mut machine = Machine::new(vec![5]);
    let mut program = Program::new();

    loop {
        match program.process(&mut codes, &mut machine) {
            NextInstruction::Halt => break,
            NextInstruction::Continue(p) => program = p,
        }
    }

    if machine.outputs.iter().filter(|&&o| o != 0).count() > 1 {
        panic!("test program failed");
    };

    machine.outputs.pop().expect("program produced no output")
}

#[derive(Debug, Clone, Copy)]
struct Intcode(i32);

impl Intcode {
    pub fn new(code: &str) -> Self {
        Intcode(code.parse().expect("could not parse code into an integer"))
    }

    pub fn as_position(&self) -> usize {
        self.0.try_into().expect("could not parse into a position")
    }
}

enum NextInstruction {
    Continue(Program),
    Halt,
}

#[derive(Debug)]
struct Machine {
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl Machine {
    fn new(inputs: Vec<i32>) -> Self {
        Machine {
            inputs,
            outputs: vec![],
        }
    }
}

#[derive(Debug)]
struct Program {
    ip: usize,
    awaiting: Awaiting,
}

impl Program {
    fn new() -> Self {
        Program {
            ip: 0,
            awaiting: Awaiting::Instruction(AwaitingInstruction::new()),
        }
    }

    fn process<'s>(self, codes: &mut [Intcode], machine: &mut Machine) -> NextInstruction {
        let code = codes[self.ip];

        let program = match self.awaiting {
            Awaiting::Instruction(ai) => match ai.provide(code, machine) {
                ProgramState::Halted => return NextInstruction::Halt,
                ProgramState::Running(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                },
            },
            Awaiting::Params(ap) => Program {
                awaiting: Awaiting::SingleParam(ap.provide(code, codes)),
                ip: self.ip + 1,
            },
            Awaiting::SingleParam(sp) => match sp.provide(code, codes, machine) {
                AwaitOrJump::Await(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                },
                AwaitOrJump::Jump(ip) => Program {
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                    ip,
                },
            },
            Awaiting::SaveLocation(value) => {
                let address = code.as_position();
                codes[address] = Intcode(value);
                Program {
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                    ip: self.ip + 1,
                }
            }
        };

        NextInstruction::Continue(program)
    }
}

#[derive(Debug)]
enum Awaiting {
    Instruction(AwaitingInstruction),
    Params(AwaitingParams),
    SingleParam(AwaitingSingle),
    SaveLocation(i32),
}

#[derive(Debug)]
struct AwaitingInstruction {}

impl AwaitingInstruction {
    fn new() -> Self {
        AwaitingInstruction {}
    }

    fn provide(self, code: Intcode, machine: &mut Machine) -> ProgramState {
        let instruction: u32 = (code.0).try_into().expect("could not parse instruction");
        let opcode = instruction % 100;

        let param_type = |param_number: u32| {
            let code = (instruction / 10u32.pow(param_number + 1)) % 10;
            if code == 0 {
                ParamType::Position
            } else {
                ParamType::Immediate
            }
        };

        let awaiting = match opcode {
            1 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Add,
            }),
            2 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Multiply,
            }),
            3 => {
                let value = machine
                    .inputs
                    .pop()
                    .expect("program ended waiting for input");
                Awaiting::SaveLocation(value)
            }
            4 => Awaiting::SingleParam(AwaitingSingle {
                param: param_type(1),
                operation: UnaryOperation::Output,
            }),
            5 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::JumpIfTrue,
            }),
            6 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::JumpIfFalse,
            }),
            7 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::LessThan,
            }),
            8 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Equal,
            }),
            99 => return ProgramState::Halted,
            opcode => unimplemented!("opcode: '{}' not yet implemented", opcode),
        };

        ProgramState::Running(awaiting)
    }
}

#[derive(Debug)]
struct AwaitingParams {
    first_param: ParamType,
    second_param: ParamType,
    operation: Operation,
}

impl AwaitingParams {
    fn provide(self, code: Intcode, codes: &mut [Intcode]) -> AwaitingSingle {
        let param = match self.first_param {
            ParamType::Immediate => code.0,
            ParamType::Position => {
                let position = code.as_position();
                codes[position].0
            }
        };

        let operation = match self.operation {
            Operation::Add => UnaryOperation::Add(param),
            Operation::Multiply => UnaryOperation::Multiply(param),
            Operation::JumpIfTrue => UnaryOperation::Jump(param != 0),
            Operation::JumpIfFalse => UnaryOperation::Jump(param == 0),
            Operation::LessThan => UnaryOperation::LessThan(param),
            Operation::Equal => UnaryOperation::Equal(param),
        };

        AwaitingSingle {
            param: self.second_param,
            operation,
        }
    }
}

#[derive(Debug)]
struct AwaitingSingle {
    param: ParamType,
    operation: UnaryOperation,
}

impl AwaitingSingle {
    fn provide(self, code: Intcode, codes: &mut [Intcode], machine: &mut Machine) -> AwaitOrJump {
        let param = match self.param {
            ParamType::Immediate => code.0,
            ParamType::Position => {
                let position = code.as_position();
                codes[position].0
            }
        };

        match self.operation {
            UnaryOperation::Output => {
                machine.outputs.push(param);
                AwaitOrJump::Await(Awaiting::Instruction(AwaitingInstruction {}))
            }
            UnaryOperation::Add(adder) => AwaitOrJump::Await(Awaiting::SaveLocation(adder + param)),
            UnaryOperation::Multiply(factor) => {
                AwaitOrJump::Await(Awaiting::SaveLocation(factor * param))
            }
            UnaryOperation::Jump(jump) => {
                if jump {
                    AwaitOrJump::Jump(param.try_into().expect("could not parse into position"))
                } else {
                    AwaitOrJump::Await(Awaiting::Instruction(AwaitingInstruction {}))
                }
            }
            UnaryOperation::LessThan(value) => {
                AwaitOrJump::Await(Awaiting::SaveLocation((value < param) as i32))
            }
            UnaryOperation::Equal(value) => {
                AwaitOrJump::Await(Awaiting::SaveLocation((value == param) as i32))
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
}

#[derive(Debug)]
enum UnaryOperation {
    Output,
    Add(i32),
    Multiply(i32),
    Jump(bool),
    LessThan(i32),
    Equal(i32),
}

#[derive(Debug)]
enum ParamType {
    Immediate,
    Position,
}

enum ProgramState {
    Running(Awaiting),
    Halted,
}

enum AwaitOrJump {
    Await(Awaiting),
    Jump(usize),
}

common::read_main!();
