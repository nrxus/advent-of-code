use std::{collections::HashMap, convert::TryInto};

pub enum MachineResult {
    AwaitingInput(AwaitingInput),
    HasOutput(HasOutput),
    Halted(Memory),
}

pub struct AwaitingInput {
    ip: usize,
    relative: usize,
    memory: Memory,
    save_param: ParamType,
}

impl AwaitingInput {
    pub fn provide(self, input: i64) -> Machine {
        Machine {
            program: Program {
                ip: self.ip + 1,
                awaiting: Awaiting::SaveLocation(input, self.save_param),
                relative: self.relative,
            },
            memory: self.memory,
        }
    }
}

pub struct HasOutput {
    output: i64,
    ip: usize,
    relative: usize,
    memory: Memory,
}

impl HasOutput {
    pub fn read(self) -> (i64, Machine) {
        (
            self.output,
            Machine {
                program: Program {
                    ip: self.ip + 1,
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                    relative: self.relative,
                },
                memory: self.memory,
            },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intcode(pub i64);

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
    memory: Memory,
}

impl Machine {
    pub fn from_str(input: &str) -> Self {
        let codes: Vec<_> = input.trim().split(",").map(Intcode::new).collect();
        Machine::new(codes)
    }

    pub fn new(codes: Vec<Intcode>) -> Self {
        Machine {
            program: Program::new(),
            memory: Memory {
                embedded: codes,
                external: HashMap::new(),
            },
        }
    }

    pub fn run_to_halt(self) -> Result<Memory, String> {
        match self.execute() {
            MachineResult::Halted(memory) => Ok(memory),
            MachineResult::HasOutput(output) => Err(format!(
                "expected a halt; got output: '{}'",
                output.read().0
            )),
            MachineResult::AwaitingInput(_) => Err("expected halt; but needs input".to_string()),
        }
    }

    pub fn execute(mut self) -> MachineResult {
        loop {
            // dbg!(&self);

            match self.program.next(&mut self.memory) {
                NextInstruction::Halt => break MachineResult::Halted(self.memory),
                NextInstruction::AwaitingInput {
                    ip,
                    relative,
                    save_param,
                } => {
                    break MachineResult::AwaitingInput(AwaitingInput {
                        ip,
                        save_param,
                        relative: relative,
                        memory: self.memory,
                    })
                }
                NextInstruction::HasOutput {
                    output,
                    ip,
                    relative,
                } => {
                    break MachineResult::HasOutput(HasOutput {
                        output,
                        ip,
                        relative,
                        memory: self.memory,
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
    AwaitingInput {
        ip: usize,
        relative: usize,
        save_param: ParamType,
    },
    HasOutput {
        output: i64,
        ip: usize,
        relative: usize,
    },
}

#[derive(Debug)]
struct Program {
    ip: usize,
    relative: usize,
    awaiting: Awaiting,
}

impl Program {
    fn new() -> Self {
        Program {
            ip: 0,
            relative: 0,
            awaiting: Awaiting::Instruction(AwaitingInstruction::new()),
        }
    }

    fn next(mut self, memory: &mut Memory) -> NextInstruction {
        let code = memory.get(self.ip);

        let program = match self.awaiting {
            Awaiting::Instruction(ai) => match ai.provide(code) {
                ProgramState::Halted => return NextInstruction::Halt,
                ProgramState::Running(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                    relative: self.relative,
                },
                ProgramState::Waiting(save_param) => {
                    return NextInstruction::AwaitingInput {
                        save_param,
                        ip: self.ip,
                        relative: self.relative,
                    }
                }
            },
            Awaiting::Params(ap) => Program {
                awaiting: Awaiting::SingleParam(ap.provide(code, &*memory, self.relative)),
                ip: self.ip + 1,
                relative: self.relative,
            },
            Awaiting::SingleParam(sp) => match sp.provide(code, &*memory, &mut self.relative) {
                PostOperation::Await(awaiting) => Program {
                    awaiting,
                    ip: self.ip + 1,
                    relative: self.relative,
                },
                PostOperation::Jump(ip) => Program {
                    awaiting: Awaiting::Instruction(AwaitingInstruction {}),
                    relative: self.relative,
                    ip,
                },
                PostOperation::Output(output) => {
                    return NextInstruction::HasOutput {
                        output,
                        ip: self.ip,
                        relative: self.relative,
                    }
                }
            },
            Awaiting::SaveLocation(value, param_type) => {
                let address = match param_type {
                    ParamType::Immediate => panic!("cannot save into an immediate"),
                    ParamType::Position => code.as_position(),
                    ParamType::Relative => (self.relative as i64 + code.0) as usize,
                };

                memory.set(address, value);
                Program {
                    relative: self.relative,
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
    SaveLocation(i64, ParamType),
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
            match code {
                0 => ParamType::Position,
                1 => ParamType::Immediate,
                2 => ParamType::Relative,
                _ => panic!("unexpected param mode"),
            }
        };

        let awaiting = match opcode {
            1 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Add(param_type(3)),
            }),
            2 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Multiply(param_type(3)),
            }),
            3 => return ProgramState::Waiting(param_type(1)),
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
                operation: Operation::LessThan(param_type(3)),
            }),
            8 => Awaiting::Params(AwaitingParams {
                first_param: param_type(1),
                second_param: param_type(2),
                operation: Operation::Equal(param_type(3)),
            }),
            9 => Awaiting::SingleParam(AwaitingSingle {
                param: param_type(1),
                operation: UnaryOperation::AdjustRelative,
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
    fn provide(self, code: Intcode, memory: &Memory, relative: usize) -> AwaitingSingle {
        let param = match self.first_param {
            ParamType::Immediate => code.0,
            ParamType::Position => {
                let position = code.as_position();
                memory.get(position).0
            }
            ParamType::Relative => {
                let position = (relative as i64 + code.0) as usize;
                memory.get(position).0
            }
        };

        let operation = match self.operation {
            Operation::Add(save_param) => UnaryOperation::Add(param, save_param),
            Operation::Multiply(save_param) => UnaryOperation::Multiply(param, save_param),
            Operation::JumpIfTrue => UnaryOperation::Jump(param != 0),
            Operation::JumpIfFalse => UnaryOperation::Jump(param == 0),
            Operation::LessThan(save_param) => UnaryOperation::LessThan(param, save_param),
            Operation::Equal(save_param) => UnaryOperation::Equal(param, save_param),
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
    fn provide(self, code: Intcode, memory: &Memory, relative: &mut usize) -> PostOperation {
        let param = match self.param {
            ParamType::Immediate => code.0,
            ParamType::Position => {
                let position = code.as_position();
                memory.get(position).0
            }
            ParamType::Relative => {
                let position = (*relative as i64 + code.0) as usize;
                memory.get(position).0
            }
        };

        match self.operation {
            UnaryOperation::Output => PostOperation::Output(param),
            UnaryOperation::AdjustRelative => {
                *relative = (*relative as i64 + param) as usize;
                PostOperation::Await(Awaiting::Instruction(AwaitingInstruction {}))
            }
            UnaryOperation::Add(adder, save_param) => {
                PostOperation::Await(Awaiting::SaveLocation(adder + param, save_param))
            }
            UnaryOperation::Multiply(factor, save_param) => {
                PostOperation::Await(Awaiting::SaveLocation(factor * param, save_param))
            }
            UnaryOperation::Jump(jump) => {
                if jump {
                    PostOperation::Jump(param.try_into().expect("could not parse into position"))
                } else {
                    PostOperation::Await(Awaiting::Instruction(AwaitingInstruction {}))
                }
            }
            UnaryOperation::LessThan(value, save_param) => {
                PostOperation::Await(Awaiting::SaveLocation((value < param) as i64, save_param))
            }
            UnaryOperation::Equal(value, save_param) => {
                PostOperation::Await(Awaiting::SaveLocation((value == param) as i64, save_param))
            }
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(ParamType),
    Multiply(ParamType),
    JumpIfTrue,
    JumpIfFalse,
    LessThan(ParamType),
    Equal(ParamType),
}

#[derive(Debug)]
enum UnaryOperation {
    Output,
    AdjustRelative,
    Add(i64, ParamType),
    Multiply(i64, ParamType),
    Jump(bool),
    LessThan(i64, ParamType),
    Equal(i64, ParamType),
}

#[derive(Debug)]
enum ParamType {
    Immediate,
    Position,
    Relative,
}

enum ProgramState {
    Running(Awaiting),
    Waiting(ParamType),
    Halted,
}

enum PostOperation {
    Await(Awaiting),
    Jump(usize),
    Output(i64),
}

#[derive(Debug)]
pub struct Memory {
    embedded: Vec<Intcode>,
    external: HashMap<usize, Intcode>,
}

impl Memory {
    pub fn get(&self, pos: usize) -> Intcode {
        self.embedded
            .get(pos)
            .or_else(|| self.external.get(&pos))
            .cloned()
            .unwrap_or(Intcode(0))
    }

    fn set(&mut self, pos: usize, value: i64) {
        match self.embedded.get_mut(pos) {
            Some(v) => *v = Intcode(value),
            None => {
                self.external.insert(pos, Intcode(value));
            }
        }
    }
}
