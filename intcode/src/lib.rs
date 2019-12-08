use std::convert::TryInto;

pub enum MachineResult {
    AwaitingInput(AwaitingInput),
    HasOutput(HasOutput),
    Halted(Vec<Intcode>),
}

pub struct AwaitingInput {
    ip: usize,
    codes: Vec<Intcode>,
}

impl AwaitingInput {
    pub fn provide(self, input: i32) -> Machine {
        Machine {
            program: Program {
                ip: self.ip + 1,
                awaiting: Awaiting::SaveLocation(input),
            },
            codes: self.codes,
        }
    }
}

pub struct HasOutput {
    output: i32,
    ip: usize,
    codes: Vec<Intcode>,
}

impl HasOutput {
    pub fn read(self) -> (i32, Machine) {
        (
            self.output,
            Machine {
                program: Program {
                    ip: self.ip + 1,
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                },
                codes: self.codes,
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intcode(pub i32);

impl Intcode {
    pub fn new(code: &str) -> Self {
        Intcode(code.parse().expect("could not parse code into an integer"))
    }

    fn as_position(&self) -> usize {
        self.0.try_into().expect("could not parse into a position")
    }
}

#[derive(Debug)]
pub struct Machine {
    program: Program,
    codes: Vec<Intcode>,
}

impl Machine {
    pub fn new(codes: Vec<Intcode>) -> Self {
        Machine {
            program: Program::new(),
            codes,
        }
    }

    pub fn execute(mut self) -> MachineResult {
        loop {
            match self.program.next(&mut self.codes) {
                NextInstruction::Halt => break MachineResult::Halted(self.codes),
                NextInstruction::AwaitingInput(ip) => {
                    break MachineResult::AwaitingInput(AwaitingInput {
                        ip,
                        codes: self.codes,
                    })
                }
                NextInstruction::HasOutput(output, ip) => {
                    break MachineResult::HasOutput(HasOutput {
                        output,
                        ip,
                        codes: self.codes,
                    })
                }
                NextInstruction::Continue(p) => self.program = p,
            }
        }
    }
}

enum NextInstruction {
    Continue(Program),
    Halt,
    AwaitingInput(usize),
    HasOutput(i32, usize),
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

    fn next(self, codes: &mut [Intcode]) -> NextInstruction {
        let code = codes[self.ip];

        let program = match self.awaiting {
            Awaiting::Instruction(ai) => match ai.provide(code) {
                ProgramState::Halted => return NextInstruction::Halt,
                ProgramState::Running(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                },
                ProgramState::Waiting => return NextInstruction::AwaitingInput(self.ip),
            },
            Awaiting::Params(ap) => Program {
                awaiting: Awaiting::SingleParam(ap.provide(code, codes)),
                ip: self.ip + 1,
            },
            Awaiting::SingleParam(sp) => match sp.provide(code, &*codes) {
                PostOperation::Await(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                },
                PostOperation::Jump(ip) => Program {
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                    ip,
                },
                PostOperation::Output(output) => {
                    return NextInstruction::HasOutput(output, self.ip)
                }
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

    fn provide(self, code: Intcode) -> ProgramState {
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
            3 => return ProgramState::Waiting,
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
    fn provide(self, code: Intcode, codes: &[Intcode]) -> PostOperation {
        let param = match self.param {
            ParamType::Immediate => code.0,
            ParamType::Position => {
                let position = code.as_position();
                codes[position].0
            }
        };

        match self.operation {
            UnaryOperation::Output => PostOperation::Output(param),
            UnaryOperation::Add(adder) => {
                PostOperation::Await(Awaiting::SaveLocation(adder + param))
            }
            UnaryOperation::Multiply(factor) => {
                PostOperation::Await(Awaiting::SaveLocation(factor * param))
            }
            UnaryOperation::Jump(jump) => {
                if jump {
                    PostOperation::Jump(param.try_into().expect("could not parse into position"))
                } else {
                    PostOperation::Await(Awaiting::Instruction(AwaitingInstruction {}))
                }
            }
            UnaryOperation::LessThan(value) => {
                PostOperation::Await(Awaiting::SaveLocation((value < param) as i32))
            }
            UnaryOperation::Equal(value) => {
                PostOperation::Await(Awaiting::SaveLocation((value == param) as i32))
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
    Waiting,
    Halted,
}

enum PostOperation {
    Await(Awaiting),
    Jump(usize),
    Output(i32),
}
